use approx::{self, assert_relative_eq};
use theta_chart::*;

#[test]
fn line_new() {
    let line = Line::new(Point::new(3., 4.), Vector::new(1., 1.));
    let end_point = line.get_end_point();
    assert_relative_eq!(Point::new(4., 5.), end_point);
}
