extern crate serde;
extern crate serde_json;

use std::any::Any;
use serde::Deserialize;

pub struct Node {
    id: String,
    name: String,
    visible: bool,
    r#type: Any,
}

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NodeType {
    DOCUMENT,
    CANVAS,
    FRAME,
    GROUP,
    VECTOR,
    BOOLEAN_OPERATION,
    STAR,
    LINE,
    ELLIPSE,
    REGULAR_POLYGON,
    RECTANGLE,
    TEXT,
    SLICE,
    COMPONENT,
    INSTANCE
}

#[derive(Clone, Deserialize, Debug)]
pub struct Color {
    pub a: f64,
    pub b: f64,
    pub g: f64,
    pub r: f64,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Deserialize, Debug)]
pub enum BlendMode {
    // Normal blends
    PASS_THROUGH,
    NORMAL,

    // Darken
    DARKEN,
    MULTIPLY,
    LINEAR_BURN,
    COLOR_BURN,

    // Lighten
    LIGHTEN,
    SCREEN,
    LINEAR_DODGE,
    COLOR_DODGE,

    // Contrast
    OVERLAY,
    SOFT_LIGHT,
    HARD_LIGHT,

    // Inversion
    DIFFERENCE,
    EXCLUSION,

    // Component
    HUE,
    SATURATION,
    COLOR,
    LUMINOSITY,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Deserialize, Debug)]
pub struct ColorStop {
    pub position: f64,
    pub color: Color,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Paint {
    pub r#type: String,
    pub visible: bool,
    pub opacity: f64,
    pub color: Color,
    pub gradientHandlePositions: Vec<Vector>,
    pub gradientStops: Vec<ColorStop>,
    pub scaleMode: String,
    // imageTransform: Transform
}

#[derive(Clone, Deserialize, Debug)]
pub struct TypeStyle {
    pub fontFamily: String,
    pub fontPostScriptName: String,
    pub italic: bool,
    pub fontWeight: f64,
    pub fontSize: f64,
    pub textDecoration: String,
    pub textAlignHorizontal: String,
    pub textAlignVertical: String,
    pub letterSpacing: f64,
    pub fills: Vec<Paint>,
    pub lineHeightPx: f64,
    pub lineHeightPercent: f64,
}

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Fill {
    pub blend_mode: String,
    pub r#type: String,
}

#[derive(Clone, Deserialize, Debug)]
pub enum StyleType {
    FILL,
    TEXT,
    EFFECT,
    GRID,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Style {
    // keyString
    // The unique identifier of the style
    pub key: String,
    // file_keyString
    // The unique identifier of the file which contains the style
    pub file_key: String,
    // node_idString
    // Id of the style node within the figma file
    pub node_id: String,
    // style_typeStyleType
    // The type of style
    pub style_type: StyleType,
    // thumbnail_urlString
    // URL link to the style's thumbnail image
    pub thumbnail_url: String,
    // nameString
    // Name of the style
    pub name: String,
    // descriptionString
    // The description of the style as entered by the publisher
    pub description: String,
    // created_atString
    // The UTC ISO 8601 time at which the style was created
    pub created_at: String,
    // updated_atString
    pub updated_at: String,
    // The UTC ISO 8601 time at which the style was updated
}

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Document {
    pub id: String,
    pub name: String,
    pub fills: Vec<Fill>,
    pub r#type: NodeType,
}

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub document: Document,
    pub schema_version: i32,
}
