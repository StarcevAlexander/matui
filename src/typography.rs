use crate::render::Render;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextStyle {
    DisplayLarge,
    DisplayMedium,
    DisplaySmall,
    HeadlineLarge,
    HeadlineMedium,
    HeadlineSmall,
    TitleLarge,
    TitleMedium,
    TitleSmall,
    BodyLarge,
    BodyMedium,
    BodySmall,
    LabelLarge,
    LabelMedium,
    LabelSmall,
}

impl TextStyle {
    pub fn class(self) -> &'static str {
        match self {
            TextStyle::DisplayLarge => "matui-display-large",
            TextStyle::DisplayMedium => "matui-display-medium",
            TextStyle::DisplaySmall => "matui-display-small",
            TextStyle::HeadlineLarge => "matui-headline-large",
            TextStyle::HeadlineMedium => "matui-headline-medium",
            TextStyle::HeadlineSmall => "matui-headline-small",
            TextStyle::TitleLarge => "matui-title-large",
            TextStyle::TitleMedium => "matui-title-medium",
            TextStyle::TitleSmall => "matui-title-small",
            TextStyle::BodyLarge => "matui-body-large",
            TextStyle::BodyMedium => "matui-body-medium",
            TextStyle::BodySmall => "matui-body-small",
            TextStyle::LabelLarge => "matui-label-large",
            TextStyle::LabelMedium => "matui-label-medium",
            TextStyle::LabelSmall => "matui-label-small",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Text {
    text: String,
    style: TextStyle,
}

impl Text {
    pub fn new(text: impl Into<String>, style: TextStyle) -> Self {
        Self {
            text: text.into(),
            style,
        }
    }
}

impl Render for Text {
    fn render(&self) -> String {
        format!(
            "<span class=\"{}\">{}</span>",
            self.style.class(),
            self.text
        )
    }
}
