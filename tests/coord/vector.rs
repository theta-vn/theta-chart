use approx::{self, assert_relative_eq};
use theta_chart::{coord::*, TAU};

#[test]
fn vector_rotate() {
    let vector = Vector::new(3., 4.);

    let new = vector.az_rotate_tau(0.25 * TAU);
    // let pr = point1;
    // dbg!(new);
    assert_relative_eq!(Vector::new(-4., 3.), new);

    // let new = Point::new(-3., -4.000_000_000_01);
    // assert_relative_eq!(pr, new);
}
