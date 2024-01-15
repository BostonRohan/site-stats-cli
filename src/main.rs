use process_url::process_url;
use process_robots_txt::process_robots_txt;
use std::{
    error::Error,
    io::{self, Write},
};

mod process_url;
mod process_robots_txt;

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

    //add a / to the end of the url if it isn't there
    if input.chars().last().unwrap() != '/' {
        input = input + "/";
    }

    //add url prefix if not there - assume we're using https
    if !input.starts_with("https://") {
        input = "https://".to_owned() + &input;
    }

    process_url(&input).await?;
    process_robots_txt(&input).await?;


    Ok(())
}
