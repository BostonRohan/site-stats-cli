use reqwest;
use std::error::Error;

pub async fn process_robots_txt(input: &str) -> Result<(), Box<dyn Error>>{
    //ping the robots.txt file and see if it exists on the following site
    if !reqwest::get(format!("{}{}", input, "robots.txt"))
    .await?
    .status()
    .is_success()
{
    println!("The following site does not have a robots.txt file, if this is your site - it is advised that you create one. \nhttps://developers.google.com/search/docs/crawling-indexing/robots/intro#:~:text=A%20robots.txt%20file%20tells,or%20password%2Dprotect%20the%20page. ");
}
        //TODO: read that robots txt file to find a sitemap(s)

Ok(())
}