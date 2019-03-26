#[macro_use]
extern crate tera;
extern crate clap;

mod figma;
mod design;

use std::collections::{HashSet, HashMap};
use std::{env, process};

use failure::Error;

use env_logger;
use log::{info, debug};
use clap::{App, Arg, SubCommand};

use figma::Node;
use design::Source;

const ACCESS_TOKEN_KEY: &str = "FIGMA_ACCESS_TOKEN";

fn main() -> Result<(), Error> {
    env_logger::init();

    let access_token = match env::var(ACCESS_TOKEN_KEY) {
        Ok(val) => val,
        Err(err) => {
            println!("{}: {}", err, ACCESS_TOKEN_KEY);
            process::exit(1);
        },
    };

    let matches = App::new("exporter")
        .version("0.0.1")
        .author("Yu Tawata <yuta24@bivre.com>")
        .about("Figma style exporter")
        .arg(Arg::with_name("team-id")
            .help("Set figma's team id")
            .short("t")
            .long("team-id")
            .takes_value(true)
        )
        .subcommand(SubCommand::with_name("generate")
            .about("generate code")
        )
        .get_matches();

    let team_id = match matches.value_of("team-id") {
        Some(team_id) => team_id,
        None => {
            println!("Set figma's team id");
            process::exit(1);
        },
    };

    let client = figma::Client::new(access_token);

    let styles = r#try!(client.get_styles(&team_id));

    let file_keys = styles.iter().map(|style| {
        return style.file_key.clone();
    });

    let file_keys: HashSet<String> = file_keys.into_iter().collect();
    let mut file_nodes = HashMap::new();
    for file_key in file_keys {
    let file_styles: Vec<String> = styles.clone().into_iter().filter(|style| style.file_key == *file_key).map(|style| style.node_id).collect();
        file_nodes.insert(file_key, file_styles);
    }

    debug!("{:#?}", file_nodes);

    let mut responses = Vec::new();
    for (file_key, file_node_ids) in file_nodes.iter() {
        let nodes = r#try!(client.get_file_nodes(&file_key, &file_node_ids));
        responses.push(nodes);
    }

    let nodes: Vec<Node> = responses.iter()
        .flat_map(|response| response)
        .cloned()
        .collect();

    let mut source = Source { rects: Vec::new(), texts: Vec::new() };
    for node in nodes {
        match node {
            Node::Rectangle { r } => {
                source.rects.push(r);
            },
            Node::Text { t } => {
                source.texts.push(t);
            },
        }
    }

    debug!("{:#?}", source);

    source.generate();

    Ok(())
}
