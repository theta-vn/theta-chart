use chrono::{Duration, NaiveDateTime};

use crate::{
    color::Color,
    coord::{Arc, Axes, Point},
};

/// This trait allows to create a number axes for the chart
pub trait ScaleNumber {
    // min, max, max_abs
    fn domain(&self) -> (f64, f64);
    fn count_distance_step(&self) -> (f64, f64, f64);
    fn to_percent(&self) -> Vec<f64>;
    // For Pie
    fn gen_pie(&self, origin: Point, radius: f64) -> Vec<Arc>;
    fn get_intervale(&self, len: f64) -> f64;
    // fn gen_sticks_label_step(&self) -> (Vec<String>, f64);
    fn gen_axes(&self) -> Axes;
}

/// This trait allows to create a label axes for the chart
pub trait ScaleLabel {
    // fn labels(&self) -> Vec<String>;
    fn colors(&self) -> Vec<Color>;
    fn get_intervale(&self, len: f64) -> f64;
    // fn gen_sticks_label_step(&self) -> (Vec<String>, f64);
    fn gen_axes(&self) -> Axes;
}

/// This trait allows to create a time axes for the chart
pub trait ScaleTime {
    fn series(&self) -> Vec<NaiveDateTime>;
    fn domain(&self) -> (NaiveDateTime, NaiveDateTime);
    // fn domain_unit(&self) -> (NaiveDateTime, NaiveDateTime);
    fn count_distance_step(&self) -> (f64, f64);
    fn get_intervale(&self, len: f64) -> f64;
    fn gen_sticks_label_step(&self) -> (Vec<(String)>, f64);
    fn scale_intervale(&self, value: NaiveDateTime) -> f64;
}

pub trait ScaleType {
    fn scale_type(&self) -> String;
}
