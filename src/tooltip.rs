use crate::render::Render;

#[derive(Clone, Debug)]
pub struct Tooltip {
    content: String,
    tip: String,
}

impl Tooltip {
    pub fn new(content: impl Into<String>, tip: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            tip: tip.into(),
        }
    }
}

impl Render for Tooltip {
    fn render(&self) -> String {
        format!(
            "<span class=\"matui-tooltip\">{}<span class=\"matui-tooltip__bubble\">{}</span></span>",
            self.content, self.tip
        )
    }
}
