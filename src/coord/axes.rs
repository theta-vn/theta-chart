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
    // pub fn reverse(&self) -> Self {
    //     let len = self.sticks.len();
    //     let sticks = &self.sticks;
    //     let mut vec: Vec<Stick> = vec![];
    //     for index in 0..len {
    //         vec.push(sticks[index].set_value(sticks[len - 1 - index].value));
    //     }
    //     Self {
    //         sticks: vec,
    //         step: self.step,
    //     }
    // }
}
