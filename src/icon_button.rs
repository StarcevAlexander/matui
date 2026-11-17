use crate::render::Render;
use crate::theme::Color;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum IconButtonVariant {
    #[default]
    Standard,
    Filled,
    Tonal,
    Outlined,
}

impl IconButtonVariant {
    pub fn class_suffix(self) -> &'static str {
        match self {
            IconButtonVariant::Standard => "standard",
            IconButtonVariant::Filled => "filled",
            IconButtonVariant::Tonal => "tonal",
            IconButtonVariant::Outlined => "outlined",
        }
    }
}

#[derive(Clone, Debug)]
pub struct IconButton {
    icon: String,
    variant: IconButtonVariant,
    color: Color,
    aria: String,
    disabled: bool,
    href: Option<String>,
}

impl IconButton {
    pub fn new(icon: impl Into<String>) -> Self {
        let icon = icon.into();
        Self {
            aria: icon.clone(),
            icon,
            variant: IconButtonVariant::Standard,
            color: Color::Primary,
            disabled: false,
            href: None,
        }
    }

    pub fn variant(mut self, variant: IconButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn aria(mut self, label: impl Into<String>) -> Self {
        self.aria = label.into();
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

impl Render for IconButton {
    fn render(&self) -> String {
        let classes = format!(
            "matui-icon-btn matui-icon-btn--{} matui-icon-btn--{}",
            self.variant.class_suffix(),
            self.color.class_suffix()
        );
        let disabled_attr = if self.disabled { " disabled" } else { "" };
        match &self.href {
            Some(href) => format!(
                "<a class=\"{classes}\" href=\"{href}\" aria-label=\"{}\">{}</a>",
                self.aria, self.icon
            ),
            None => format!(
                "<button class=\"{classes}\" type=\"button\"{disabled_attr} aria-label=\"{}\">{}</button>",
                self.aria, self.icon
            ),
        }
    }
}
