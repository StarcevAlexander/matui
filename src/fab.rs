use crate::render::Render;
use crate::theme::Color;

#[derive(Clone, Debug)]
pub struct Fab {
    icon: String,
    label: Option<String>,
    color: Color,
    disabled: bool,
}

impl Fab {
    pub fn new(icon: impl Into<String>) -> Self {
        Self {
            icon: icon.into(),
            label: None,
            color: Color::Primary,
            disabled: false,
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Render for Fab {
    fn render(&self) -> String {
        let classes = format!("matui-fab matui-fab--{}", self.color.class_suffix());
        let disabled_attr = if self.disabled { " disabled" } else { "" };
        let label = self
            .label
            .as_ref()
            .map(|l| format!("<span class=\"matui-fab__label\">{l}</span>"))
            .unwrap_or_default();
        format!(
            "<button class=\"{classes}\" type=\"button\"{disabled_attr} aria-label=\"{}\"><span class=\"matui-fab__icon\" aria-hidden=\"true\">{}</span>{label}</button>",
            self.icon, self.icon
        )
    }
}
