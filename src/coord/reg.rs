use crate::coord::{Point, Vector};
#[derive(Debug, Clone, Default)]
/// Store data for rectangle on chart
pub struct Rec {
    origin: Point,
    vector: Vector,
}

impl Rec {
    pub fn new(origin: Point, vector: Vector) -> Self {
        Self { origin, vector }
    }

    pub fn get_origin(&self) -> Point {
        self.origin.clone()
    }

    pub fn get_vector(&self) -> Vector {
        self.vector.clone()
    }

    pub fn get_width(&self) -> f64 {
        self.vector.get_x().abs()
    }

    pub fn get_height(&self) -> f64 {
        self.vector.get_y().abs()
    }
}
