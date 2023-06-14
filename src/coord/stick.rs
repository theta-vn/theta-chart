#[derive(Debug, Clone, Default)]
/// Store data for 1 stick rectangle on chart
pub struct Stick {
    pub label: String,
    pub value: f64,
}

impl Stick {
    pub fn new(label: String, value: f64) -> Self {
        Self { label, value }
    }

    pub fn set_value(&self, value: f64) -> Self {
        Self {
            label: self.label.clone(),
            value,
        }
    }
}
