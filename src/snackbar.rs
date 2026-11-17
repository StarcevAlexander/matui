use crate::render::Render;

#[derive(Clone, Debug)]
pub struct Snackbar {
    message: String,
    action: Option<String>,
    open: bool,
}

impl Snackbar {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            action: None,
            open: false,
        }
    }

    pub fn action(mut self, label: impl Into<String>) -> Self {
        self.action = Some(label.into());
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }
}

impl Render for Snackbar {
    fn render(&self) -> String {
        let mut classes = String::from("matui-snackbar");
        if !self.open {
            classes.push_str(" matui-snackbar--hidden");
        }
        let action = self
            .action
            .as_ref()
            .map(|a| {
                format!("<button class=\"matui-snackbar__action\" type=\"button\">{a}</button>")
            })
            .unwrap_or_default();
        format!(
            "<div class=\"{classes}\" role=\"status\">{}{action}</div>",
            self.message
        )
    }
}
