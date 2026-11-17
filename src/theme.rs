#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    Primary,
    Secondary,
    Tertiary,
    Error,
}

impl Color {
    pub fn class_suffix(self) -> &'static str {
        match self {
            Color::Primary => "primary",
            Color::Secondary => "secondary",
            Color::Tertiary => "tertiary",
            Color::Error => "error",
        }
    }
}
