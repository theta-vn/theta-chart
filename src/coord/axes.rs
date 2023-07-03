use crate::coord::Stick;

#[derive(Debug, Clone, Default)]
/// Store data for one Axes
pub struct Axes {
    pub sticks: Vec<Stick>,
    pub step: f64,
    pub style: String,
}

impl Axes {
    pub fn new(sticks: Vec<Stick>, step: f64, style: String) -> Self {
        Self {
            sticks,
            step,
            style,
        }
    }
}
