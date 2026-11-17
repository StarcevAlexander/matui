use crate::render::Render;

#[derive(Clone, Debug)]
pub struct Slider {
    name: String,
    min: f64,
    max: f64,
    value: f64,
    step: f64,
    disabled: bool,
}

impl Slider {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            min: 0.0,
            max: 100.0,
            value: 50.0,
            step: 1.0,
            disabled: false,
        }
    }

    pub fn range(mut self, min: f64, max: f64) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = value;
        self
    }

    pub fn step(mut self, step: f64) -> Self {
        self.step = step;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Render for Slider {
    fn render(&self) -> String {
        let span = if (self.max - self.min).abs() < f64::EPSILON {
            0.0
        } else {
            (self.value - self.min) / (self.max - self.min) * 100.0
        };
        let disabled_attr = if self.disabled { " disabled" } else { "" };
        format!(
            "<input type=\"range\" class=\"matui-slider\" name=\"{name}\" min=\"{min}\" max=\"{max}\" step=\"{step}\" value=\"{value}\"{disabled_attr} style=\"--matui-slider-value: {span:.0}%;\">",
            name = self.name,
            min = self.min,
            max = self.max,
            step = self.step,
            value = self.value,
        )
    }
}
