use crate::render::Render;

#[derive(Clone, Debug, Default)]
pub struct Divider {
    inset: bool,
    vertical: bool,
}

impl Divider {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn inset(mut self, inset: bool) -> Self {
        self.inset = inset;
        self
    }

    pub fn vertical(mut self, vertical: bool) -> Self {
        self.vertical = vertical;
        self
    }
}

impl Render for Divider {
    fn render(&self) -> String {
        let mut classes = String::from("matui-divider");
        if self.inset {
            classes.push_str(" matui-divider--inset");
        }
        if self.vertical {
            classes.push_str(" matui-divider--vertical");
        }
        format!("<div role=\"separator\" class=\"{classes}\"></div>")
    }
}
