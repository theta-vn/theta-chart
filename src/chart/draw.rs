use crate::coord::{Point, Rec};

use super::Category;

/// This trait allows to create a view for the chart
pub trait Draw {
    fn get_region(&self) -> Rec;
    fn origin(&self) -> Point;
    fn width(&self) -> f64;
    fn height(&self) -> f64;
    fn chart_type(&self) -> Category;
}
