mod figma;

use std::collections::HashSet;
use std::collections::HashMap;
use std::env;
use std::process;

use env_logger;
use log::info;

const TEAM_ID_KEY: &str = "FIGMA_TEAM_ID";
const ACCESS_TOKEN_KEY: &str = "FIGMA_ACCESS_TOKEN";

fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let access_token = match env::var(ACCESS_TOKEN_KEY) {
        Ok(val) => val,
        Err(err) => {
            println!("{}: {}", err, ACCESS_TOKEN_KEY);
            process::exit(1);
        },
    };

    let client = figma::Client::new(access_token);

    let team_id = match env::var(TEAM_ID_KEY) {
        Ok(val) => val,
        Err(err) => {
            println!("{}: {}", err, TEAM_ID_KEY);
            process::exit(1);
        },
    };

    let styles = client.get_styles(&team_id);

    let file_nodes = styles.map( |styles| {
        let file_keys = styles.iter().map(|style| {
            return style.file_key.clone();
        });
        let file_keys: HashSet<String> = file_keys.into_iter().collect();
        let mut file_nodes = HashMap::new();
        for file_key in file_keys {
        let file_styles: Vec<String> = styles.clone().into_iter().filter(|style| style.file_key == *file_key).map(|style| style.node_id).collect();
            file_nodes.insert(file_key, file_styles);
        }
        return file_nodes;
    });

    info!("{:#?}", file_nodes);

    match file_nodes {
        Ok(res) => {
            for (file_key, file_node_ids) in res.iter() {
                match client.get_file_nodes(&file_key, &file_node_ids) {
                    Ok(res) => {
                        info!("{:#?}", res);
                    },
                    Err(err) => {
                        info!("{:#?}", err);
                    },
                };
            }
        },
        Err(err) => {
            info!("{:#?}", err);
        },
    };
}
