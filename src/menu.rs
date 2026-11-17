use crate::render::Render;

#[derive(Clone, Debug)]
pub struct MenuItem {
    label: String,
    icon: Option<String>,
}

impl MenuItem {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: None,
        }
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct Menu {
    items: Vec<MenuItem>,
    open: bool,
}

impl Menu {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn item(mut self, item: MenuItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }
}

impl Render for Menu {
    fn render(&self) -> String {
        let mut classes = String::from("matui-menu");
        if !self.open {
            classes.push_str(" matui-menu--hidden");
        }
        let mut out = format!("<div class=\"{classes}\" role=\"menu\">");
        for item in &self.items {
            let icon = item
                .icon
                .as_ref()
                .map(|i| format!("<span aria-hidden=\"true\">{i}</span>"))
                .unwrap_or_default();
            out.push_str(&format!(
                "<button class=\"matui-menu__item\" type=\"button\" role=\"menuitem\">{icon}{}</button>",
                item.label
            ));
        }
        out.push_str("</div>");
        out
    }
}
