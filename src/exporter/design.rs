use super::figma;

use tera::Context;

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

impl Source {
    pub fn generate(&self) {
        match &self.style_type {
            StyleType::Text => {
                self.generate_text();
            },
            StyleType::Color => {
                self.generate_color();
            },
        };
    }

    fn generate_color(&self) {
        let tera = tera::compile_templates!("templates/**/*");

        let mut ctx = Context::new();
        let mut rects = self.rects.clone();
        rects.sort_by( |a, b| a.name.cmp(&b.name));
        ctx.insert("rects", &rects);

        let rendered = tera.render("swift/ColorStyle.swift", &ctx).expect("Failed to render template");
        println!("{}", rendered);
    }

    fn generate_text(&self) {
        let tera = tera::compile_templates!("templates/**/*");

        let mut ctx = Context::new();
        let mut texts = self.texts.clone();
        texts.sort_by( |a, b| a.name.cmp(&b.name));
        ctx.insert("texts", &texts);

        let rendered = tera.render("swift/TextStyle.swift", &ctx).expect("Failed to render template");
        println!("{}", rendered);
    }
}
