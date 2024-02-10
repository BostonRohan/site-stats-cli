use process_robots_txt::process_robots_txt;
use process_sitemaps::process_sitemaps;
use process_url::process_url;
use std::{
    error::Error,
    io::{self, Write},
};

mod process_robots_txt;
mod process_sitemaps;
mod process_url;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Welcome to site stats!");

    let mut input = String::new();

    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    print!("Enter your site url:");

    io::stdout()
        .flush()
        .expect("There was an error writing to the input");

    io::stdin()
        .read_line(&mut input)
        .expect("There was an error reading the input");

    input = input.trim().to_string();

    //add a / to the end of the url if it isn't there
    if input.chars().last().unwrap() != '/' {
        input = input + "/";
    }

    //add url prefix if not there - assume we're using https
    if !input.starts_with("https://") {
        input = "https://".to_owned() + &input;
    }

    process_url(&input).await?;
    //TODO: ask if the user wants these tasks, maybe they only want to look at the direct url they pasted and not the base site
    let robots_sitemaps = process_robots_txt(&input).await?;

    //TODO: This should run on another thread
    let pages = process_sitemaps(&robots_sitemaps).await;
    info!("{}: {:?} {}", "Your site has", pages.0.len() + 1, "pages");
    if pages.1.len() > 0 {
        warn!("{:?} {}", pages.1.len(), "of which are unreachable.")
    } else {
        info!("{:?} {}", pages.1.len(), "of which are unreachable.");
    }

    Ok(())
}
