use crate::render::Render;

#[derive(Clone, Debug, Default)]
pub struct Card {
    title: Option<String>,
    body: String,
    elevated: bool,
}

impl Card {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = body.into();
        self
    }

    pub fn elevated(mut self, elevated: bool) -> Self {
        self.elevated = elevated;
        self
    }
}

impl Render for Card {
    fn render(&self) -> String {
        let mut classes = String::from("matui-card");
        if self.elevated {
            classes.push_str(" matui-card--elevated");
        }
        let title = self
            .title
            .as_ref()
            .map(|t| format!("<h3 class=\"matui-card__title\">{t}</h3>"))
            .unwrap_or_default();
        format!(
            "<div class=\"{classes}\">{title}<div class=\"matui-card__body\">{}</div></div>",
            self.body
        )
    }
}
