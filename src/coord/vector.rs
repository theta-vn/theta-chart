use na::{Isometry3, Vector3};
use nalgebra as na;

use super::{point::Point3D, Point};

pub type Vector3D = Vector3<f64>;
#[derive(Debug, Clone, PartialEq)]
/// Store data for vector on chart
pub struct Vector(Vector3D);

impl Default for Vector {
    fn default() -> Self {
        Self(Vector3::new(0., 0., 1.))
    }
}

impl From<(Point, Point)> for Vector {
    fn from(value: (Point, Point)) -> Self {
        Self(Vector3::new(
            value.1.get_x() - value.0.get_x(),
            value.1.get_y() - value.0.get_y(),
            1.,
        ))
    }
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Self {
        Self(Vector3::new(x, y, 1.))
    }

    fn get(&self) -> Vector3D {
        self.0
    }

    pub fn get_x(&self) -> f64 {
        self.0.x
    }

    pub fn get_y(&self) -> f64 {
        self.0.y
    }

    pub fn set_x(&self, x: f64) -> Self {
        Self(Vector3::new(x, self.0.y, self.0.z))
    }

    pub fn set_y(&self, y: f64) -> Self {
        Self(Vector3::new(self.0.x, y, self.0.z))
    }
    pub fn to_point(&self) -> Point {
        let point = Point3D::from(self.get());
        Point::new(point.x, point.y)
    }

    pub fn module(&self) -> f64 {
        (self.get_x().powf(2.) + self.get_y().powf(2.)).sqrt()
    }

    pub fn az_rotate_tau(&self, tau: f64) -> Vector {
        let axisangle = Vector3::z() * tau;
        let iso = Isometry3::new(Vector3::default(), axisangle);
        let m = iso * self.get();
        Vector(m)
    }

    pub fn multiply(&self, num: f64) -> Vector {
        Vector(num * self.get())
    }
}

impl approx::AbsDiffEq for Vector {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        1.0e-6
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        Vector3D::abs_diff_eq(&self.0, &other.0, epsilon)
    }

    fn abs_diff_ne(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        !Self::abs_diff_eq(self, other, epsilon)
    }
}

impl approx::RelativeEq for Vector {
    fn default_max_relative() -> Self::Epsilon {
        1.0e-6
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        Vector3D::relative_eq(&self.0, &other.0, epsilon, max_relative)
    }
}
