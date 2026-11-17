use crate::render::Render;

#[derive(Clone, Debug)]
pub struct SearchBar {
    placeholder: String,
    value: String,
    name: String,
}

impl SearchBar {
    pub fn new(placeholder: impl Into<String>) -> Self {
        Self {
            placeholder: placeholder.into(),
            value: String::new(),
            name: "search".into(),
        }
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }
}

impl Render for SearchBar {
    fn render(&self) -> String {
        let value_attr = if self.value.is_empty() {
            String::new()
        } else {
            format!(" value=\"{}\"", self.value)
        };
        format!(
            "<div class=\"matui-search\"><span aria-hidden=\"true\">⌕</span><input type=\"search\" name=\"{}\" placeholder=\"{}\"{value_attr}></div>",
            self.name, self.placeholder
        )
    }
}
