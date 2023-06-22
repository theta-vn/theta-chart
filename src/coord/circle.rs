use crate::coord::Point;
#[derive(Debug, Clone, Default)]
/// Store data for rectangle on chart
pub struct Circle {
    origin: Point,
    radius: f64,
}

impl Circle {
    pub fn new(origin: Point, radius: f64) -> Self {
        Self { origin, radius }
    }

    pub fn get_origin(&self) -> Point {
        self.origin.clone()
    }

    pub fn get_radius(&self) -> f64 {
        self.radius
    }
}
