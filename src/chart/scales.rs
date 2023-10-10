use chrono::NaiveDateTime;

use crate::coord::{Arc, Axes, Stick, Vector};

/// This trait allows to create a label axes for the chart
pub trait ScaleLabel {
    // fn colors(&self) -> Vec<Color>;
    fn scale(&self, value: f64) -> f64;
    fn scale_index(&self, label: String) -> usize;
    fn gen_axes(&self) -> Axes;
    fn to_stick(&self) -> Vec<Stick>;
}
/// This trait allows to create a number axes for the chart
pub trait ScaleNumber {
    // min, max
    fn domain(&self) -> (f64, f64);
    fn scale(&self, value: f64) -> f64;
    fn count_distance_step(&self) -> (f64, f64, f64);
    fn to_percent(&self) -> Vec<f64>;

    // For Pie
    fn gen_pie(&self) -> Vec<Arc>;
    fn to_percent_radar(&self) -> Vec<f64>;    
    fn gen_axes(&self) -> Axes;

    // To stick for series
    fn to_stick(&self) -> Vec<Stick>;

    // For Radar
    fn gen_radar_grid(&self, count: usize) -> Vec<Vector>;
}

/// This trait allows to create a time axes for the chart
pub trait ScaleTime {
    fn domain(&self) -> (NaiveDateTime, NaiveDateTime);
    fn domain_unix(&self) -> (f64, f64);
    fn scale(&self, value: NaiveDateTime) -> f64;
    // fn domain_unit(&self) -> (NaiveDateTime, NaiveDateTime);
    // fn count_distance_step(&self) -> (f64, f64);
    // fn get_intervale(&self, len: f64) -> f64;
    // fn scale_intervale(&self, value: NaiveDateTime) -> f64;
    fn gen_axes(&self) -> Axes;
    // To stick for series
    fn to_stick(&self) -> Vec<Stick>;
}
