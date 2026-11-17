use crate::render::Render;

#[derive(Clone, Debug, Default)]
pub struct BottomSheet {
    content: String,
    open: bool,
}

impl BottomSheet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = content.into();
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }
}

impl Render for BottomSheet {
    fn render(&self) -> String {
        let mut classes = String::from("matui-bottom-sheet");
        if !self.open {
            classes.push_str(" matui-bottom-sheet--hidden");
        }
        format!(
            "<div class=\"{classes}\"><div class=\"matui-bottom-sheet__handle\"></div>{}</div>",
            self.content
        )
    }
}
