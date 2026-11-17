use crate::render::Render;

#[derive(Clone, Debug, Default)]
pub struct BottomAppBar {
    leading: Option<String>,
    trailing: Option<String>,
}

impl BottomAppBar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn leading(mut self, content: impl Into<String>) -> Self {
        self.leading = Some(content.into());
        self
    }

    pub fn trailing(mut self, content: impl Into<String>) -> Self {
        self.trailing = Some(content.into());
        self
    }
}

impl Render for BottomAppBar {
    fn render(&self) -> String {
        let leading = self
            .leading
            .as_ref()
            .map(|l| format!("<div class=\"matui-bottom-appbar__section\">{l}</div>"))
            .unwrap_or_default();
        let trailing = self
            .trailing
            .as_ref()
            .map(|t| format!("<div class=\"matui-bottom-appbar__section\">{t}</div>"))
            .unwrap_or_default();
        format!("<footer class=\"matui-bottom-appbar\">{leading}{trailing}</footer>")
    }
}
