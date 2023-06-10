use approx::{self, assert_relative_eq};
use theta_chart::coord::*;

#[test]
fn point_rotate() {
    let point = Point::new(3., 4.);

    let point1 = point.rotate_turn(0.5);
    let pr = point1;

    assert_relative_eq!(Point::new(-3., -4.), pr);

    let new = Point::new(-3., -4.000_000_000_01);
    assert_relative_eq!(pr, new);
}
