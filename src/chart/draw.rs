use crate::coord::{Point, Vector};

use super::Category;

/// This trait allows to create a view for the chart
pub trait Draw {
    // Origin of region inner
    fn get_origin(&self) -> Point;

    // Center of region inner
    fn get_center(&self) -> Point;

    // Width and heigth of all region draw
    fn get_outer(&self) -> Vector;

    // Width and heigth of chart
    fn get_inner(&self) -> Vector;

    // Radius of inscribed circle
    fn get_radius(&self) -> f64;

    // // Radius of inscribed circle
    // fn get_region_axes(&self, position: usize) -> Rec;

    fn chart_type(&self) -> Category;
}
