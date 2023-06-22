use crate::coord::Stick;

#[derive(Debug, Clone, Default)]
/// Store data for 1 Axes
pub struct Axes {
    pub sticks: Vec<Stick>,
    pub step: f64,
}

impl Axes {
    pub fn new(sticks: Vec<Stick>, step: f64) -> Self {
        Self { sticks, step }
    }
}
