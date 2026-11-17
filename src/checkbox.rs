use crate::render::Render;

#[derive(Clone, Debug)]
pub struct Checkbox {
    label: String,
    checked: bool,
    indeterminate: bool,
    disabled: bool,
}

impl Checkbox {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            checked: false,
            indeterminate: false,
            disabled: false,
        }
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn indeterminate(mut self, indeterminate: bool) -> Self {
        self.indeterminate = indeterminate;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Render for Checkbox {
    fn render(&self) -> String {
        let checked_attr = if self.checked { " checked" } else { "" };
        let indeterminate_attr = if self.indeterminate {
            " data-indeterminate"
        } else {
            ""
        };
        let disabled_attr = if self.disabled { " disabled" } else { "" };
        format!(
            "<label class=\"matui-checkbox\"><input type=\"checkbox\"{checked_attr}{indeterminate_attr}{disabled_attr}><span class=\"matui-checkbox__box\"></span>{}</label>",
            self.label
        )
    }
}
