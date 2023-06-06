use theta_chart::*;
use approx::{self, assert_relative_eq};

#[test]
fn rec_new() {
    let rec =  Rec::new(Point::new(2.,3.), Vector::new(3., 4.)) ;
    let width = rec.get_width();
    let height = rec.get_height();


    assert_relative_eq!(Point::new(2., 3.), rec.get_origin());
    assert_eq!(3., width);
    assert_eq!(4., height);
}