use std::io::{self, Write};
use url::Url;

fn main() -> io::Result<()> {
    println!("Welcome to site stats!");

    let mut input = String::new();


    print!("Enter your site url:");

    io::stdout().flush().expect("There was an error writing to the input");

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("You entered: {}", input);
            input = input.trim().to_string();

            if Url::parse(&input).is_ok() {
                println!("valid url");
            } else {
                println!("invalid url");
            }
            Ok(())
        }
        Err(error) => {
            eprintln!("Error reading input: {}", error);
            Err(error)
        }
    }
}