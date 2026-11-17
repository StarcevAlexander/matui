use crate::render::Render;

pub use crate::navigation_bar::NavItem;

#[derive(Clone, Debug)]
pub struct NavigationRail {
    header: Option<String>,
    items: Vec<NavItem>,
}

impl NavigationRail {
    pub fn new() -> Self {
        Self {
            header: None,
            items: Vec::new(),
        }
    }

    pub fn header(mut self, header: impl Into<String>) -> Self {
        self.header = Some(header.into());
        self
    }

    pub fn item(mut self, item: NavItem) -> Self {
        self.items.push(item);
        self
    }
}

impl Default for NavigationRail {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for NavigationRail {
    fn render(&self) -> String {
        let mut out = String::from("<nav class=\"matui-rail\">");
        if let Some(header) = &self.header {
            out.push_str(&format!(
                "<span class=\"matui-rail__header\">{header}</span>"
            ));
        }
        for item in &self.items {
            let mut classes = String::from("matui-rail__item");
            if item.active {
                classes.push_str(" matui-rail__item--active");
            }
            out.push_str(&format!(
                "<button class=\"{classes}\" type=\"button\"><span class=\"matui-rail__item-icon\">{}</span><span>{}</span></button>",
                item.icon, item.label
            ));
        }
        out.push_str("</nav>");
        out
    }
}
