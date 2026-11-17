use crate::render::Render;

pub use crate::navigation_bar::NavItem;

#[derive(Clone, Debug)]
pub struct NavigationDrawer {
    header: Option<String>,
    items: Vec<NavItem>,
    open: bool,
}

impl NavigationDrawer {
    pub fn new() -> Self {
        Self {
            header: None,
            items: Vec::new(),
            open: true,
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

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }
}

impl Default for NavigationDrawer {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for NavigationDrawer {
    fn render(&self) -> String {
        let mut classes = String::from("matui-drawer");
        if !self.open {
            classes.push_str(" matui-drawer--closed");
        }
        let mut out = format!("<nav class=\"{classes}\">");
        if let Some(header) = &self.header {
            out.push_str(&format!(
                "<div class=\"matui-drawer__header\">{header}</div>"
            ));
        }
        for item in &self.items {
            let mut item_classes = String::from("matui-drawer__item");
            if item.active {
                item_classes.push_str(" matui-drawer__item--active");
            }
            out.push_str(&format!(
                "<button class=\"{item_classes}\" type=\"button\"><span aria-hidden=\"true\">{}</span><span>{}</span></button>",
                item.icon, item.label
            ));
        }
        out.push_str("</nav>");
        out
    }
}
