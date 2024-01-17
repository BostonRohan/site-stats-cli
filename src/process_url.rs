use reqwest;
use url::Url;
use std::error::Error;


pub async fn process_url(input: &str) -> Result<(), Box<dyn Error>> {
    let url = Url::parse(input)
        .map_err(|e| format!("There was an issue with the following url: {}", e))?;

    let response = reqwest::get(url.clone()).await;

    //ping the base url to see if it is a valid website
    if response.is_err() || !response?.status().is_success() {
        return Err("The site that was provided either does not exist or cannot be reached at the moment, please try again.".into());
    }

    Ok(())
}