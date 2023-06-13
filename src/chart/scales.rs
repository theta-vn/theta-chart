use chrono::{NaiveDateTime, Duration};

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
    // For Pie
    fn gen_pie(&self, origin: Point, radius: f64) -> Vec<Arc>;    
    fn get_interval(&self, len: f64) -> f64;
    fn gen_sticks_label_step(&self) -> (Vec<String>, f64);
}

/// This trait allows to create a label axes for the chart
pub trait ScaleLabel {
    fn labels(&self) -> Vec<String>;
    fn colors(&self) -> Vec<Color>;
    fn get_interval(&self, len: f64) -> f64;
}


/// This trait allows to create a time axes for the chart
pub trait ScaleTime {
    fn series(&self) -> Vec<NaiveDateTime>;    
    fn domain(&self) -> (NaiveDateTime, NaiveDateTime);
    fn count_distance_step(&self) -> (usize, Duration, usize);    
    fn get_interval(&self, len: f64) -> f64;

}

pub trait ScaleType {
    fn scale_type(&self) -> String;
}
