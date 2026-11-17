use crate::render::Render;

#[derive(Clone, Debug)]
pub struct TopAppBar {
    title: String,
    actions: Option<String>,
}

impl TopAppBar {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            actions: None,
        }
    }

    pub fn actions(mut self, actions: impl Into<String>) -> Self {
        self.actions = Some(actions.into());
        self
    }
}

impl Render for TopAppBar {
    fn render(&self) -> String {
        let actions = self
            .actions
            .as_ref()
            .map(|a| format!("<div class=\"matui-topbar__actions\">{a}</div>"))
            .unwrap_or_default();
        format!(
            "<header class=\"matui-topbar\"><h1 class=\"matui-topbar__title\">{}</h1>{actions}</header>",
            self.title
        )
    }
}
