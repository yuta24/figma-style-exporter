use reqwest;
use serde;
use serde::{Serialize, Deserialize};
use serde_json::Value;

const BASE_URL: &str = "https://api.figma.com";

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum StyleType {
    FILL,
    TEXT,
    EFFECT,
    GRID,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Style {
    pub key: String,
    pub file_key: String,
    pub node_id: String,
    pub style_type: StyleType,
    pub thumbnail_url: String,
    pub name: String,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
    // pub user: User,
    pub sort_position: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Paint {
    pub r#type: String,
    pub blend_mode: String,
    pub color: Color,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TypeStyle {
    font_family: String,
    font_post_script_name: String,
    italic: Option<bool>,
    font_weight: f64,
    font_size: f64,
    text_decoration: Option<String>,
    text_align_horizontal: String,
    text_align_vertical: String,
    letter_spacing: f64,
    fills: Option<Vec<Paint>>,
    line_height_px: f64,
    line_height_percent: f64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RectangleNode {
    pub id: String,
    pub name: String,
    pub blend_mode: String,
    pub fills: Vec<Paint>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TextNode {
    pub id: String,
    pub name: String,
    pub blend_mode: String,
    pub style: TypeStyle
}

#[derive(Clone, Debug)]
pub enum Node {
    Rectangle { r: RectangleNode },
    Text { t: TextNode },
}

pub struct Client {
    access_token: String
}

impl Client {
    pub fn new(access_token: String) -> Client {
        return Client { access_token: access_token};
    }

    pub fn get_styles(&self, team_id: &str) -> Result<Vec<Style>, reqwest::Error> {
        let client = reqwest::Client::new();
        let url = format!("{}/v1/teams/{}/styles", BASE_URL, team_id);

        return client.get(&url).header("X-FIGMA-TOKEN", self.access_token.clone()).send().map( |mut res| {
            let text = res.text().unwrap();
            let json: Value = serde_json::from_str(&text).unwrap();
            let styles = json["meta"]["styles"].clone();
            let styles: Vec<Style> = serde_json::from_value(styles).unwrap();
            return styles;
        });
    }

    pub fn get_file_nodes(&self, file_key: &String, node_ids: &Vec<String>) -> Result<Vec<Node>, reqwest::Error> {
        let client = reqwest::Client::new();
        let url = format!("{}/v1/files/{}/nodes", BASE_URL, file_key);
        let query = [("ids", node_ids.join(","))];

        return client.get(&url).query(&query).header("X-FIGMA-TOKEN", self.access_token.clone()).send().map( |mut res| {
            let text = res.text().unwrap();
            let json: Value = serde_json::from_str(&text).unwrap();

            let mut nodes = Vec::new();
            for (_, value) in json["nodes"].as_object().unwrap().iter() {
                match value["document"]["type"].as_str() {
                    Some(str) => match str {
                    "RECTANGLE" => {
                        let rectangle: RectangleNode = serde_json::from_value(value["document"].clone()).unwrap();
                        let node = Node::Rectangle { r: rectangle };
                        nodes.push(node);
                    },
                    "TEXT" => {
                        let text: TextNode = serde_json::from_value(value["document"].clone()).unwrap();
                        let node = Node::Text { t: text };
                        nodes.push(node);
                    },
                    _ => {},
                    },
                    None => {},
                }
            }

            return nodes;
        });
    }
}
