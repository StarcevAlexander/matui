use crate::render::Render;

#[derive(Clone, Debug)]
pub struct Badge {
    text: Option<String>,
    dot: bool,
}

impl Badge {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: Some(text.into()),
            dot: false,
        }
    }

    pub fn dot() -> Self {
        Self {
            text: None,
            dot: true,
        }
    }
}

impl Render for Badge {
    fn render(&self) -> String {
        if self.dot {
            return String::from("<span class=\"matui-badge matui-badge--dot\"></span>");
        }
        format!(
            "<span class=\"matui-badge\">{}</span>",
            self.text.as_deref().unwrap_or("")
        )
    }
}
