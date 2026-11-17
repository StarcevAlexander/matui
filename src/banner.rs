use crate::render::Render;

#[derive(Clone, Debug)]
pub struct Banner {
    text: String,
    icon: Option<String>,
    actions: Option<String>,
}

impl Banner {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            icon: None,
            actions: None,
        }
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn actions(mut self, actions: impl Into<String>) -> Self {
        self.actions = Some(actions.into());
        self
    }
}

impl Render for Banner {
    fn render(&self) -> String {
        let icon = self
            .icon
            .as_ref()
            .map(|i| format!("<span class=\"matui-banner__icon\" aria-hidden=\"true\">{i}</span>"))
            .unwrap_or_default();
        let actions = self
            .actions
            .as_ref()
            .map(|a| format!("<div class=\"matui-banner__actions\">{a}</div>"))
            .unwrap_or_default();
        format!(
            "<div class=\"matui-banner\">{icon}<div class=\"matui-banner__text\">{}{actions}</div></div>",
            self.text
        )
    }
}
