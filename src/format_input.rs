use dialoguer::{theme::ColorfulTheme, Select};

pub fn format_input(input: String) -> String {
    //remove leading and trailing whitespace
    input.trim().to_string();

    if input.starts_with("http") || input.starts_with("https") {
        if input.starts_with("http") && !input.contains("localhost") {
            info!("If this is your site it is recommended that you use https: https://www.cloudflare.com/learning/ssl/why-use-https/")
        }
        input
    } else {
        //add protocol prefix if not there - ask the user if the site is http or https
        const PROTOCOL_OPTIONS: [&str; 2] = ["no", "yes"];
        let protocol_option = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Is this site using http:")
            .items(&PROTOCOL_OPTIONS[..])
            .default(0)
            .interact();

        match protocol_option {
            Ok(protocol_option) => {
                let protocol = if PROTOCOL_OPTIONS[protocol_option] == "yes" {
                    "http"
                } else {
                    "https"
                };
                format!("{}://{input}", protocol)
            }
            Err(e) => {
                warn!("There was an issue providing options to pick a protocol type, as a result we will default to https");
                debug!("{:?}", e);
                format!("https://{input}")
            }
        }
    }
}
