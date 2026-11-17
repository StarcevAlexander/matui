use crate::render::Render;

#[derive(Clone, Debug)]
pub struct SegmentOption {
    text: String,
    icon: Option<String>,
}

impl SegmentOption {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            icon: None,
        }
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct SegmentedButtonGroup {
    options: Vec<SegmentOption>,
    selected: Option<usize>,
    density: bool,
}

impl SegmentedButtonGroup {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn option(mut self, option: SegmentOption) -> Self {
        self.options.push(option);
        self
    }

    pub fn selected(mut self, index: usize) -> Self {
        self.selected = Some(index);
        self
    }

    pub fn density(mut self, density: bool) -> Self {
        self.density = density;
        self
    }
}

impl Render for SegmentedButtonGroup {
    fn render(&self) -> String {
        let mut out = String::from("<div class=\"matui-segmented\" role=\"group\">");
        for (i, opt) in self.options.iter().enumerate() {
            let mut classes = String::from("matui-segmented__btn");
            if self.selected == Some(i) {
                classes.push_str(" matui-segmented__btn--selected");
            }
            if self.density {
                classes.push_str(" matui-segmented__btn--dense");
            }
            let icon = opt
                .icon
                .as_ref()
                .map(|ic| format!("<span>{ic}</span>"))
                .unwrap_or_default();
            out.push_str(&format!(
                "<button class=\"{classes}\" type=\"button\" aria-pressed=\"{}\">{icon}{}</button>",
                self.selected == Some(i),
                opt.text
            ));
        }
        out.push_str("</div>");
        out
    }
}
