use crate::{Point, Vector};
#[derive(Debug)]
pub struct Rec {
    origin: Point,
    vector: Vector
}

impl Rec {
    pub fn new(origin: Point, vector: Vector) -> Self {
        Self { origin, vector }
    }

    pub fn get_origin(&self) -> Point {
        self.origin.clone()
    }

    pub fn get_width(&self) -> f64 {
        self.vector.get_x()
    }

    pub fn get_height(&self) -> f64 {
        self.vector.get_y()
    }
}