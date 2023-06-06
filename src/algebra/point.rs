use nalgebra as na;
use na::{Point3, Vector3, Isometry3};


use super::TAU;

#[derive(Debug, Clone)]
pub struct Point(Point3<f64>) ;

impl Default for Point {
    fn default() -> Self {
        Self(Point3::new(0., 0., 1.))
    }    
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self(Point3::new(x, y, 1.))
    }

    pub fn value(&self) -> Point3<f64> {
        self.0
    }

    pub fn get_x(&self) -> f64 {
        self.0.x
    }

    pub fn get_y(&self) -> f64 {
        self.0.y
    }

    pub fn set_x(&mut self, x: f64) -> Self {
        self.0.x = x;
        self.clone()
    } 
    
    pub fn set_y(&mut self, y: f64) -> Self {
        self.0.y = y;
        self.clone()
    }

    pub fn rotate(self, ratio: f64) -> Point {
        let arc = ratio * TAU;
        let axisangle = Vector3::z() * arc;
        let iso = Isometry3::new(Vector3::default(), axisangle);
        let p = iso * self.value();
        Point(p)
    }
}



