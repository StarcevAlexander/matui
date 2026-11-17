use crate::render::Render;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TextFieldVariant {
    #[default]
    Filled,
    Outlined,
}

impl TextFieldVariant {
    pub fn class_suffix(self) -> &'static str {
        match self {
            TextFieldVariant::Filled => "filled",
            TextFieldVariant::Outlined => "outlined",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct TextField {
    label: String,
    name: String,
    value: String,
    variant: TextFieldVariant,
    helper: Option<String>,
    error: Option<String>,
    disabled: bool,
}

impl TextField {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            name: String::new(),
            value: String::new(),
            variant: TextFieldVariant::Filled,
            helper: None,
            error: None,
            disabled: false,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn variant(mut self, variant: TextFieldVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn helper(mut self, helper: impl Into<String>) -> Self {
        self.helper = Some(helper.into());
        self
    }

    pub fn error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Render for TextField {
    fn render(&self) -> String {
        let mut classes = format!(
            "matui-textfield matui-textfield--{}",
            self.variant.class_suffix()
        );
        if self.error.is_some() {
            classes.push_str(" matui-textfield--error");
        }
        let name_attr = if self.name.is_empty() {
            String::new()
        } else {
            format!(" name=\"{}\"", self.name)
        };
        let value_attr = if self.value.is_empty() {
            String::new()
        } else {
            format!(" value=\"{}\"", self.value)
        };
        let disabled_attr = if self.disabled { " disabled" } else { "" };
        let helper_text = self.error.clone().or_else(|| self.helper.clone());
        let helper = helper_text
            .map(|t| format!("<span class=\"matui-textfield__helper\">{t}</span>"))
            .unwrap_or_default();
        format!(
            "<div class=\"{classes}\"><input class=\"matui-textfield__input\" type=\"text\"{name_attr}{value_attr}{disabled_attr} placeholder=\" \"><label class=\"matui-textfield__label\">{}</label>{helper}</div>",
            self.label
        )
    }
}
