use na::{Isometry3, Point3, Vector3};
use nalgebra as na;

use crate::Point;

#[derive(Debug, Clone)]
/// Store data for vector on chart
pub struct Vector(Vector3<f64>);

impl Default for Vector {
    fn default() -> Self {
        Self(Vector3::new(0., 0., 1.))
    }
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Self {
        Self(Vector3::new(x, y, 1.))
    }
    fn get(&self) -> Vector3<f64> {
        self.0
    }

    pub fn get_x(&self) -> f64 {
        self.0.x
    }

    pub fn get_y(&self) -> f64 {
        self.0.y
    }

    pub fn to_point(&self) -> Point {
        let point = Point3::from(self.get());
        Point::new(point.x, point.y)
    }

    pub fn module(&self) -> f64 {
        (self.get_x().powf(2.) + self.get_y().powf(2.)).sqrt()
    }

    pub fn az_rotate(self, angle: f64) -> Vector {
        let axisangle = Vector3::z() * angle;
        let iso = Isometry3::new(Vector3::default(), axisangle);
        let m = iso * self.get();
        // Point(p)
        // let v = Vector3::from(m.fixed_view(0, 3));
        // log::debug!("Vector_end:::{:#?}", m);
        // Vector::default()
        Vector(m)
    }
}
