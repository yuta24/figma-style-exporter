#[macro_use]
extern crate tera;
extern crate clap;
extern crate inflector;

mod figma;
mod exporter;

use std::{env, process};

use failure::Error;

use env_logger;
use clap::{App, Arg};

const ACCESS_TOKEN_KEY: &str = "FIGMA_ACCESS_TOKEN";

fn main() -> Result<(), Error> {
    env_logger::init();

    let matches = App::new("exporter")
        .version("0.0.5")
        .author("Yu Tawata <yuta24@bivre.com>")
        .about("Figma style exporter")
        .arg(Arg::with_name("team-id")
            .help("Set figma's team id")
            .long("team-id")
            .required(true)
            .takes_value(true)
        )
        .arg(Arg::with_name("style")
            .help("style type")
            .long("style-type")
            .required(true)
            .takes_value(true)
        )
        .arg(Arg::with_name("template")
            .help("template path")
            .long("template-path")
            .required(true)
            .takes_value(true)
        )
        .get_matches();

    let team_id = match matches.value_of("team-id") {
        Some(team_id) => team_id,
        None => {
            println!("Set figma's team id");
            process::exit(1);
        },
    };
    let style_type = match matches.value_of("style") {
        Some(style_type) => style_type,
        None => {
            println!("Set style type");
            process::exit(1);
        },
    };
    let template_path = match matches.value_of("template") {
        Some(template_path) => template_path,
        None => {
            println!("Set template path");
            process::exit(1);
        },
    };

    let access_token = match env::var(ACCESS_TOKEN_KEY) {
        Ok(val) => val,
        Err(err) => {
            println!("{}: {}", err, ACCESS_TOKEN_KEY);
            process::exit(1);
        },
    };

    let exporter = exporter::Exporter::new(access_token, team_id.to_string(), style_type.to_string(), template_path.to_string());
    exporter.execute()
}
