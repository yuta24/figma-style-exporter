mod design;

use std::str::FromStr;
use std::result::Result;
use std::collections::{HashSet, HashMap};

use log::debug;
use failure::Error;

use super::figma;
use super::figma::Node;
use self::design::Source;
use self::design::StyleType;

impl FromStr for StyleType {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "text" => Ok(StyleType::Text),
            "color" => Ok(StyleType::Color),
            _ => Err("Error"),
        }
    }
}

pub struct Exporter {
    access_token: String,
    team_id: String,
    style_type: StyleType,
}

impl Exporter {
    pub fn new(access_token: String, team_id: String, style_type: String) -> Exporter {
        return Exporter { access_token: access_token, team_id: team_id, style_type: StyleType::from_str(&style_type).unwrap() };
    }

    pub fn execute(&self) -> Result<(), Error> {
        let client = figma::Client::new(self.access_token.clone());

        let styles = client.get_styles(&self.team_id)?;

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
            let nodes = client.get_file_nodes(&file_key, &file_node_ids)?;
            responses.push(nodes);
        }

        let nodes: Vec<Node> = responses.iter()
            .flat_map(|response| response)
            .cloned()
            .collect();

        let mut source = Source { style_type: self.style_type.clone(), rects: Vec::new(), texts: Vec::new() };
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
}
