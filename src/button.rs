use crate::render::Render;
use crate::theme::Color;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonVariant {
    Filled,
    Tonal,
    Outlined,
    Text,
}

impl ButtonVariant {
    pub fn class_suffix(self) -> &'static str {
        match self {
            ButtonVariant::Filled => "filled",
            ButtonVariant::Tonal => "tonal",
            ButtonVariant::Outlined => "outlined",
            ButtonVariant::Text => "text",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Button {
    label: String,
    variant: ButtonVariant,
    color: Color,
    disabled: bool,
    href: Option<String>,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            variant: ButtonVariant::Filled,
            color: Color::Primary,
            disabled: false,
            href: None,
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
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

    pub fn link(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }
}

impl Render for Button {
    fn render(&self) -> String {
        let mut classes = format!(
            "matui-btn matui-btn--{} matui-btn--{}",
            self.variant.class_suffix(),
            self.color.class_suffix()
        );
        if self.disabled {
            classes.push_str(" matui-btn--disabled");
        }
        let disabled_attr = if self.disabled { " disabled" } else { "" };
        match &self.href {
            Some(href) => format!(
                "<a class=\"{classes}\" href=\"{href}\"{disabled_attr}>{}</a>",
                self.label
            ),
            None => format!(
                "<button class=\"{classes}\" type=\"button\"{disabled_attr}>{}</button>",
                self.label
            ),
        }
    }
}
