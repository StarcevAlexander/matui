use crate::render::Render;

#[derive(Clone, Debug, Default)]
pub struct LinearProgress {
    value: Option<f32>,
}

impl LinearProgress {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn value(mut self, value: f32) -> Self {
        self.value = Some(value.clamp(0.0, 1.0));
        self
    }
}

impl Render for LinearProgress {
    fn render(&self) -> String {
        match self.value {
            Some(v) => format!(
                "<div class=\"matui-progress matui-progress--determinate\" role=\"progressbar\" aria-valuenow=\"{}\" style=\"--matui-progress-value: {v};\"><div class=\"matui-progress__bar\"></div></div>",
                (v * 100.0).round() as i32
            ),
            None => String::from(
                "<div class=\"matui-progress\" role=\"progressbar\"><div class=\"matui-progress__bar\"></div></div>",
            ),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Spinner {
    size: u32,
}

impl Spinner {
    pub fn new() -> Self {
        Self { size: 40 }
    }

    pub fn size(mut self, px: u32) -> Self {
        self.size = px.clamp(16, 96);
        self
    }
}

impl Render for Spinner {
    fn render(&self) -> String {
        format!(
            "<div class=\"matui-spinner\" role=\"status\" aria-label=\"loading\" style=\"width: {}px; height: {}px;\"></div>",
            self.size, self.size
        )
    }
}
