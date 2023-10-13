use na::{Isometry3, Point3, Vector3};
use nalgebra as na;
use robust::orient2d;

use crate::{degree_to_radian, turn_to_radian};

use super::Vector;

pub type Point3D = Point3<f64>;

#[derive(Debug, Clone, PartialEq)]
/// Store data for a point on chart
pub struct Point(Point3D);

impl Default for Point {
    fn default() -> Self {
        Self(Point3::new(0., 0., 1.))
    }
}

impl From<Point3D> for Point {
    fn from(value: Point3D) -> Self {
        Point(value)
    }
}

impl From<&Point> for robust::Coord<f64> {
    fn from(p: &Point) -> robust::Coord<f64> {
        robust::Coord::<f64> { x: p.get_x(), y: p.get_y() }
    }
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self(Point3::new(x, y, 1.))
    }

    pub fn value(&self) -> Point3D {
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

    pub fn rotate_tau(&self, tau: f64) -> Point {
        let axisangle = Vector3::z() * tau;
        let iso = Isometry3::new(Vector3::default(), axisangle);
        let p = iso * self.value();
        Point(p)
    }

    pub fn rotate_turn(&self, turn: f64) -> Point {
        let tau = turn_to_radian(turn);
        self.rotate_tau(tau)
    }

    pub fn rotate_degree(&self, degree: f64) -> Point {
        let tau = degree_to_radian(degree);
        self.rotate_tau(tau)
    }

    pub fn translate(&self, v: &Vector) -> Point {
        Point::new(self.get_x() + v.get_x(), self.get_y() + v.get_y())
    }

    // For delaunator
    pub fn dist2(&self, p: &Self) -> f64 {
        let dx = self.0.x - p.0.x;
        let dy = self.0.y - p.0.y;
        dx * dx + dy * dy
    }
    pub fn orient(&self, q: &Self, r: &Self) -> f64 {
        // robust-rs orients Y-axis upwards, our convention is Y downwards. This means that the interpretation of the result must be flipped
        orient2d(self.into(), q.into(), r.into())
    }


    pub fn circumdelta(&self, b: &Self, c: &Self) -> (f64, f64) {
        let dx = b.0.x - self.0.x;
        let dy = b.0.y - self.0.y;
        let ex = c.0.x - self.0.x;
        let ey = c.0.y - self.0.y;

        let bl = dx * dx + dy * dy;
        let cl = ex * ex + ey * ey;
        let d = 0.5 / (dx * ey - dy * ex);

        let x = (ey * bl - dy * cl) * d;
        let y = (dx * cl - ex * bl) * d;
        (x, y)
    }

    pub fn circumradius2(&self, b: &Self, c: &Self) -> f64 {
        let (x, y) = self.circumdelta(b, c);
        x * x + y * y
    }

    pub fn circumcenter(&self, b: &Self, c: &Self) -> Self {
        let (x, y) = self.circumdelta(b, c);    
        Self(Point3::new(self.0.x + x, self.0.y + y, 1.))
    }

    pub fn in_circle(&self, b: &Self, c: &Self, p: &Self) -> bool {
        let dx = self.0.x - p.0.x;
        let dy = self.0.y - p.0.y;
        let ex = b.0.x - p.0.x;
        let ey = b.0.y - p.0.y;
        let fx = c.0.x - p.0.x;
        let fy = c.0.y - p.0.y;

        let ap = dx * dx + dy * dy;
        let bp = ex * ex + ey * ey;
        let cp = fx * fx + fy * fy;

        dx * (ey * cp - bp * fy) - dy * (ex * cp - bp * fx) + ap * (ex * fy - ey * fx) < 0.0
    }

    pub fn midpoint(&self, b: &Self) -> Self {        
        Self(Point3::new((self.0.x + b.0.x)/2., (self.0.y + b.0.y)/2., 1.))
    }
}

impl approx::AbsDiffEq for Point {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        1.0e-6
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        Point3::<f64>::abs_diff_eq(&self.0, &other.0, epsilon)
    }

    fn abs_diff_ne(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        !Self::abs_diff_eq(self, other, epsilon)
    }
}

impl approx::RelativeEq for Point {
    fn default_max_relative() -> Self::Epsilon {
        1.0e-6
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        Point3::<f64>::relative_eq(&self.0, &other.0, epsilon, max_relative)
    }
}
