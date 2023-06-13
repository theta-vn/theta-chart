use theta_chart::series::SNumber;
use theta_chart::chart::ScaleNumber;

#[test]
fn new_series_number() {
    let linear = SNumber::new(vec![-36., 25., 10.]);
    dbg!(&linear);
}

#[test]
fn from_series_number() {
    let vi64: Vec<i64> = vec![-36, 25, 10];
    let linear = SNumber::from(vi64);
    dbg!(&linear);

    let vu64: Vec<u64> = vec![36, 25, 10];
    let linear = SNumber::from(vu64);
    dbg!(&linear);

    let vf64: Vec<f64> = vec![1., 2., 1.7];
    let linear = SNumber::new(vf64);
    let a = linear.gen_sticks_label_step();
    dbg!(a);
}
