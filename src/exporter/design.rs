use super::figma;

use tera::Context;

#[derive(Clone, Debug)]
pub struct Source {
    pub rects: Vec<figma::RectangleNode>,
    pub texts: Vec<figma::TextNode>,
}

impl Source {
    pub fn generate(&self) {
        self.generate_color();
        self.generate_text();
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
