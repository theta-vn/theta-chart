use crate::{
    color::Color,
    coord::{Arc, Point},
};

/// This trait allows to create a number axes for the chart
pub trait ScaleNumber {
    fn series(&self) -> Vec<f64>;
    fn is_float(&self) -> bool;
    fn domain(&self) -> (f64, f64);
    fn count_distance_step(&self) -> (usize, f64, usize);
    fn to_percent(&self) -> Vec<f64>;
    fn gen_pie(&self, origin: Point, radius: f64) -> Vec<Arc>;
}

/// This trait allows to create a label axes for the chart
pub trait ScaleLabel {
    fn labels(&self) -> Vec<String>;
    fn colors(&self) -> Vec<Color>;
}

pub trait ScaleType {
    fn scale_type(&self) -> String;
}
