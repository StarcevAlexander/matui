use crate::render::Render;

#[derive(Clone, Debug)]
pub struct Dialog {
    title: String,
    body: String,
    actions: Option<String>,
    open: bool,
}

impl Dialog {
    pub fn new(title: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            body: body.into(),
            actions: None,
            open: false,
        }
    }

    pub fn actions(mut self, actions: impl Into<String>) -> Self {
        self.actions = Some(actions.into());
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }
}

impl Render for Dialog {
    fn render(&self) -> String {
        let scrim_class = if self.open {
            "matui-dialog-scrim matui-dialog-scrim--open"
        } else {
            "matui-dialog-scrim"
        };
        let actions = self
            .actions
            .as_ref()
            .map(|a| format!("<div class=\"matui-dialog__actions\">{a}</div>"))
            .unwrap_or_default();
        format!(
            "<div class=\"{scrim_class}\"><div class=\"matui-dialog\" role=\"dialog\" aria-modal=\"true\"><h2 class=\"matui-dialog__title\">{}</h2><div class=\"matui-dialog__body\">{}</div>{actions}</div></div>",
            self.title, self.body
        )
    }
}
