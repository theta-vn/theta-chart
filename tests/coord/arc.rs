use approx::{self, assert_relative_eq};
use theta_chart::{coord::*, TAU};

#[test]
fn arc_new() {
    let arc_oxy = Arc::new(
        Point::new(3., 4.),
        Vector::new(0., 1.),
        Vector::new(1., 0.),
        false,
    );
    let arc_polar = Arc::new_polar(Point::new(3., 4.), Vector::new(0., 1.), -0.25 * TAU);
    assert_relative_eq!(arc_oxy.delta_xy().0, arc_polar.delta_xy().0);
    assert_relative_eq!(arc_oxy.delta_xy().1, arc_polar.delta_xy().1);
}
