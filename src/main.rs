extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate itertools;

use std::collections::HashSet;
use std::collections::HashMap;
use std::env;
use std::process;
use serde::Deserialize;
use serde_json::{Result, Value};

#[derive(Deserialize, Debug)]
enum StyleType {
    FILL,
    TEXT,
    EFFECT,
    GRID
}

#[derive(Deserialize, Debug)]
struct Style {
    // keyString
    // The unique identifier of the style
    key: String,
    // file_keyString
    // The unique identifier of the file which contains the style
    file_key: String,
    // node_idString
    // Id of the style node within the figma file
    node_id: String,
    // style_typeStyleType
    // The type of style
    style_type: StyleType,
    // thumbnail_urlString
    // URL link to the style's thumbnail image
    thumbnail_url: String,
    // nameString
    // Name of the style
    name: String,
    // descriptionString
    // The description of the style as entered by the publisher
    description: String,
    // created_atString
    // The UTC ISO 8601 time at which the style was created
    created_at: String,
    // updated_atString
    updated_at: String
    // The UTC ISO 8601 time at which the style was updated
}

fn main() {
    let base_url = "https://api.figma.com";
    let team_id_key = "FIGMA_TEAM_ID";
    let access_token_key = "FIGMA_ACCESS_TOKEN";

    let access_token = match env::var(access_token_key) {
        Ok(val) => val,
        Err(err) => {
            println!("{}: {}", err, access_token_key);
            process::exit(1);
        },
    };
    let team_id = match env::var(team_id_key) {
        Ok(val) => val,
        Err(err) => {
            println!("{}: {}", err, team_id_key);
            process::exit(1);
        },
    };

    println!("{}", access_token);

    // curl -H 'X-FIGMA-TOKEN: 9444-51cbb2a1-82d0-44e6-9e6e-40b0d4116c28' 'https://api.figma.com/v1/files/VrHGUOb752d69dFaIIEOC99x'

    let client = reqwest::Client::new();
    let url = format!("{}/v1/teams/{}/styles", base_url, team_id);
    let result = client.get(&url).header("X-FIGMA-TOKEN", access_token).send();

    match result {
        Ok(mut res) => {
            {
                let text = res.text().unwrap();
                let json: Value = serde_json::from_str(&text).unwrap();
                // println!("{}", json["meta"]["styles"]);
                let styles = json["meta"]["styles"].clone();
                let styles: Vec<Style> = serde_json::from_value(styles).unwrap();

                let mut file_keys = Vec::new();
                file_keys.extend(styles.iter().map(|style| {
                    return style.file_key.clone();
                }));
                let file_keys: HashSet<String> = file_keys.into_iter().collect();
                println!("{:#?}", file_keys);
                // for style in &styles {
                    // println!("{:?}", style);
                // }
            }
        },
        Err(err) => {
            println!("{}", err)
        },
    };
}
