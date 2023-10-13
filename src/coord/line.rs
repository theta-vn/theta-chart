use crate::coord::{Point, Vector};
#[derive(Debug, Clone)]
/// Store data for one line on chart
pub struct Line {
    origin: Point,
    vector: Vector,
}

impl Line {
    pub fn new(origin: Point, vector: Vector) -> Self {
        Self { origin, vector }
    }

    pub fn get_origin(&self) -> Point {
        self.origin.clone()
    }

    pub fn get_vector(&self) -> Vector {
        self.vector.clone()
    }

    pub fn get_end_point(&self) -> Point {
        self.origin.translate(&self.vector)
    }
}
