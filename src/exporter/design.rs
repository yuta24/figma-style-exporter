use super::figma;

use std::path::Path;
use std::collections::HashMap;

use tera::{Context, Result, Value, to_value};

use inflector::Inflector;

#[derive(Clone, Debug)]
pub enum StyleType {
    Text,
    Color,
}

#[derive(Clone, Debug)]
pub struct Source {
    pub style_type: StyleType,
    pub rects: Vec<figma::RectangleNode>,
    pub texts: Vec<figma::TextNode>,
}

fn camelcase(value: Value, _: HashMap<String, Value>) -> Result<Value> {
    let s = try_get_value!("camelcase", "value", String, value);
    Ok(to_value(s.to_camel_case()).unwrap())
}

impl Source {
    fn tera() -> tera::Tera {
        let mut tera = tera::Tera::default();
        tera.register_filter("camelcase", camelcase);
        return tera;
    }

    pub fn generate(&self, template_path: String) {
        match &self.style_type {
            StyleType::Text => {
                self.generate_text(template_path);
            },
            StyleType::Color => {
                self.generate_color(template_path);
            },
        };
    }

    fn generate_color(&self, template_path: String) {
        let path = Path::new(&template_path);
        let mut tera = Source::tera();
        tera.add_template_file(path, Some("color.swift")).unwrap();

        let mut ctx = Context::new();
        let mut rects = self.rects.clone();
        rects.sort_by( |a, b| a.name.cmp(&b.name));
        ctx.insert("rects", &rects);

        let rendered = tera.render("color.swift", &ctx).expect("Failed to render template");
        println!("{}", rendered);
    }

    fn generate_text(&self, template_path: String) {
        let path = Path::new(&template_path);
        let mut tera = Source::tera();
        tera.add_template_file(path, Some("text.swift")).unwrap();

        let mut ctx = Context::new();
        let mut texts = self.texts.clone();
        texts.sort_by( |a, b| a.name.cmp(&b.name));
        ctx.insert("texts", &texts);

        let rendered = tera.render("text.swift", &ctx).expect("Failed to render template");
        println!("{}", rendered);
    }
}
