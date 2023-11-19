use reqwest;
use std::{
    error::Error,
    io::{self, Write},
};
use url::Url;

async fn process_url(input: &str) -> Result<(), Box<dyn Error>> {
    let url = Url::parse(input)
        .map_err(|e| format!("There was an issue with the following url: {}", e))?;

    let response = reqwest::get(url.clone()).await;

    //ping the base url to see if it is a valid website
    if response.is_err() || !response?.status().is_success() {
        return Err("The site that was provided either does not exist or cannot be reached at the moment, please try again.".into());
    }

    //ping the robots.txt file and see if it exists on the following site
    if !reqwest::get(format!("{}{}", url, "robots.txt"))
        .await?
        .status()
        .is_success()
    {
        println!("The following site does not have a robots.txt file, if this is your site - it is advised that you create one. \nhttps://developers.google.com/search/docs/crawling-indexing/robots/intro#:~:text=A%20robots.txt%20file%20tells,or%20password%2Dprotect%20the%20page. ");
    }

    //TODO: read that robots txt file to find a sitemap(s)

    Ok(())
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Welcome to site stats!");

    let mut input = String::new();

    print!("Enter your site url:");

    io::stdout()
        .flush()
        .expect("There was an error writing to the input");

    io::stdin()
        .read_line(&mut input)
        .expect("There was an error reading the input");

    input = input.trim().to_string();

    process_url(&input).await?;

    Ok(())
}
