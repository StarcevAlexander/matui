use crate::render::Render;

#[derive(Clone, Debug, Default)]
pub struct Carousel {
    items: Vec<String>,
}

impl Carousel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn item(mut self, item: impl Into<String>) -> Self {
        self.items.push(item.into());
        self
    }
}

impl Render for Carousel {
    fn render(&self) -> String {
        let mut out = String::from("<div class=\"matui-carousel\">");
        for item in &self.items {
            out.push_str(&format!("<div class=\"matui-carousel__item\">{item}</div>"));
        }
        out.push_str("</div>");
        out
    }
}
