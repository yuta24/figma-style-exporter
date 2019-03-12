use std::env;
use std::process;
use http::{Request, Response};

fn main() {
    let base_url = "https://api.figma.com";
    let access_token_key = "FIGMA_ACCESS_TOKEN";

    let access_token = match env::var(access_token_key) {
        Ok(val) => val,
        Err(err) => {
            println!("{}: {}", err, access_token_key);
            process::exit(1);
        },
    };

    println!("{}", access_token)
}
