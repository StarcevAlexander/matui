use crate::render::Render;

#[derive(Clone, Debug, Default)]
pub struct ListItem {
    headline: String,
    supporting: Option<String>,
    leading: Option<String>,
    trailing: Option<String>,
}

impl ListItem {
    pub fn new(headline: impl Into<String>) -> Self {
        Self {
            headline: headline.into(),
            supporting: None,
            leading: None,
            trailing: None,
        }
    }

    pub fn supporting(mut self, supporting: impl Into<String>) -> Self {
        self.supporting = Some(supporting.into());
        self
    }

    pub fn leading(mut self, leading: impl Into<String>) -> Self {
        self.leading = Some(leading.into());
        self
    }

    pub fn trailing(mut self, trailing: impl Into<String>) -> Self {
        self.trailing = Some(trailing.into());
        self
    }
}

impl Render for ListItem {
    fn render(&self) -> String {
        let leading = self
            .leading
            .as_ref()
            .map(|l| format!("<span class=\"matui-list-item__leading\">{l}</span>"))
            .unwrap_or_default();
        let supporting = self
            .supporting
            .as_ref()
            .map(|s| format!("<div class=\"matui-list-item__supporting\">{s}</div>"))
            .unwrap_or_default();
        let trailing = self
            .trailing
            .as_ref()
            .map(|t| format!("<span class=\"matui-list-item__trailing\">{t}</span>"))
            .unwrap_or_default();
        format!(
            "<div class=\"matui-list-item\">{leading}<div class=\"matui-list-item__text\"><div class=\"matui-list-item__headline\">{}</div>{supporting}</div>{trailing}</div>",
            self.headline
        )
    }
}
