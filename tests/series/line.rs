use theta_chart::series::SNumber;

#[test]
fn create_series_number() {
    let linear = SNumber::new(vec![-36., 25., 10.]);
    dbg!(&linear);
}
