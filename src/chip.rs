use crate::render::Render;

#[derive(Clone, Debug)]
pub struct Chip {
    text: String,
    icon: Option<String>,
    selected: bool,
    disabled: bool,
}

impl Chip {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            icon: None,
            selected: false,
            disabled: false,
        }
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Render for Chip {
    fn render(&self) -> String {
        let mut classes = String::from("matui-chip");
        if self.selected {
            classes.push_str(" matui-chip--selected");
        }
        let disabled_attr = if self.disabled { " disabled" } else { "" };
        let icon = self
            .icon
            .as_ref()
            .map(|i| format!("<span class=\"matui-chip__icon\">{i}</span>"))
            .unwrap_or_default();
        format!(
            "<button class=\"{classes}\" type=\"button\"{disabled_attr}>{icon}{}</button>",
            self.text
        )
    }
}
