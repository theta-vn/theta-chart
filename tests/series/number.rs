use theta_chart::chart::ScaleNumber;
use theta_chart::series::SNumber;

#[test]
fn new_series_number() {
    let linear = SNumber::new(vec![-36., 25., 10.]);
    dbg!(&linear);
}

#[test]
fn from_series_number() {
    // let vi64: Vec<i64> = vec![-36, 25, 10];
    // let linear = SNumber::from(vi64);
    // dbg!(&linear);

    // let vu64: Vec<u64> = vec![36, 25, 10];
    // let linear = SNumber::from(vu64);
    // dbg!(&linear);

    let vf64: Vec<f64> = vec![1., 9., 1.7, 5.5, 3.5];
    let linear = SNumber::new(vf64).set_range(0.5, 11.);
    dbg!(&linear);
    let a = linear.gen_axes();
    dbg!(a);
}

#[test]
fn scale() {
    let vf64: Vec<f64> = vec![1., 1.9, -1.7];
    let linear = SNumber::new(vf64);
    let domain = linear.domain();
    dbg!(&domain);

    let a = linear.count_distance_step();

    dbg!(a);
    dbg!(linear.gen_axes());
}
