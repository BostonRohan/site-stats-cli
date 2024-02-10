use async_recursion::async_recursion;
use reqwest;
use roxmltree::Node;
use std::error::Error;
use url::Url;

#[async_recursion]
async fn process_xml(
    node: Node<'async_recursion, '_>,
    urls: &mut Vec<String>,
    unreachable_urls: &mut Vec<String>,
) {
    if node.tag_name().name() == "loc" {
        if let Some(url) = node.text() {
            let stringified_url = url.to_string();

            if let Ok(response) = reqwest::get(url).await {
                if response.status().is_success() {
                    urls.push(stringified_url);
                }
            } else {
                warn!("Could not reach page: {:?}", url);
                unreachable_urls.push(stringified_url)
            }
        }
        warn!("There was an issue getting the text content for the following node in the sitemap: {:?}", node);
    }

    // Recursively process children
    for child in node.children() {
        process_xml(child, urls, unreachable_urls).await;
    }
}

async fn get_sitemap_xml(url: Url) -> Result<String, Box<dyn Error>> {
    let response = reqwest::get(url.clone()).await;

    match response {
        Ok(response) => Ok(response.text().await?),
        Err(error) => {
            error!("There was an error fetching the sitemap: {}", error);
            Ok(String::new())
        }
    }
}

#[async_recursion]
pub async fn process_sitemaps(sitemaps: &Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut pages: Vec<String> = Vec::new();
    let mut unreachable_pages: Vec<String> = Vec::new();
    let mut nested_sitemap_urls: Vec<String> = Vec::new();

    for sitemap in sitemaps {
        let parsed_sitemap_url = Url::parse(&sitemap);

        let sitemap_xml = match parsed_sitemap_url {
            Ok(parsed_url) => parsed_url,
            Err(error) => {
                error!("There was an error parsing the sitemap URL: {}", sitemap);
                continue;
            }
        };

        let sitemap_xml = match get_sitemap_xml(sitemap_xml).await {
            Ok(xml) => xml,
            Err(error) => {
                error!(
                    "There was an issue getting the xml for the folowing sitemap URL: {}",
                    sitemap
                );
                continue;
            }
        };

        let doc = roxmltree::Document::parse(&sitemap_xml);

        if doc.is_err() {
            error!(
                "There was an error parsing the following xml file: {:?}",
                doc
            );
            break;
        }

        process_xml(doc.unwrap().root(), &mut pages, &mut unreachable_pages).await;

        for url in &pages {
            //make this better --- maybeee
            if url.contains("sitemap") && url.ends_with(".xml") {
                nested_sitemap_urls.push(url.clone());
            }
        }

        if nested_sitemap_urls.len() > 0 {
            let nested_sitemap_urls = process_sitemaps(&nested_sitemap_urls).await;

            return nested_sitemap_urls;
        }
    }
    return (pages, unreachable_pages);
}
