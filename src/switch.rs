use crate::render::Render;

#[derive(Clone, Debug)]
pub struct Switch {
    label: Option<String>,
    checked: bool,
    disabled: bool,
}

impl Switch {
    pub fn new() -> Self {
        Self {
            label: None,
            checked: false,
            disabled: false,
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Default for Switch {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for Switch {
    fn render(&self) -> String {
        let checked_attr = if self.checked { " checked" } else { "" };
        let disabled_attr = if self.disabled { " disabled" } else { "" };
        let label = self
            .label
            .as_ref()
            .map(|l| format!("<span>{l}</span>"))
            .unwrap_or_default();
        format!(
            "<label class=\"matui-switch\"><input type=\"checkbox\"{checked_attr}{disabled_attr}><span class=\"matui-switch__track\"><span class=\"matui-switch__thumb\"></span></span>{label}</label>"
        )
    }
}
