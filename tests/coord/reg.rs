use theta_chart::coord::*;

#[test]
fn reg_new() {
    let reg = Rec::new(Point::new(3., 4.), Vector::new(2., 3.));
    assert_eq!(2., reg.get_width());
    assert_eq!(3., reg.get_height());
}
