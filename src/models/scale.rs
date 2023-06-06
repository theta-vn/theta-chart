use crate::color::Color;

/// This trait allows to create a number axes for the chart
pub trait ScaleLinear {
    fn series(&self) -> Vec<f64>;
    fn is_float(&self) -> bool;
    fn domain(&self) -> (f64, f64);
    fn count_distance_step(&self) -> (usize, f64, usize);
}

/// This trait allows to create a label axes for the chart

pub trait ScaleCategory {
    fn labels(&self) -> Vec<String>;
    fn colors(&self) -> Vec<Color>;
}
