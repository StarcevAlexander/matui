use crate::render::Render;

#[derive(Clone, Debug)]
pub struct NavItem {
    pub(crate) icon: String,
    pub(crate) label: String,
    pub(crate) badge: Option<String>,
    pub(crate) active: bool,
}

impl NavItem {
    pub fn new(icon: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            icon: icon.into(),
            label: label.into(),
            badge: None,
            active: false,
        }
    }

    pub fn badge(mut self, badge: impl Into<String>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct NavigationBar {
    items: Vec<NavItem>,
}

impl NavigationBar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn item(mut self, item: NavItem) -> Self {
        self.items.push(item);
        self
    }
}

impl Render for NavigationBar {
    fn render(&self) -> String {
        let mut out = String::from("<nav class=\"matui-navbar\">");
        for item in &self.items {
            let mut classes = String::from("matui-navbar__item");
            if item.active {
                classes.push_str(" matui-navbar__item--active");
            }
            let badge = item
                .badge
                .as_ref()
                .map(|b| format!("<sup class=\"matui-badge\">{b}</sup>"))
                .unwrap_or_default();
            out.push_str(&format!(
                "<button class=\"{classes}\" type=\"button\"><span class=\"matui-navbar__item-icon\">{}</span><span>{}</span>{badge}</button>",
                item.icon, item.label
            ));
        }
        out.push_str("</nav>");
        out
    }
}
