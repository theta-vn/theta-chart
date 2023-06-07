use theta_chart::*;

#[test]
fn create_scale_line() {
    let linear = SeriesNumber::new(vec![-36., 25., 10.]);
    // let series = linear.series();
    // let domain = linear.domain();

    let new = linear.set_stick(7);

    // dbg!(&new);

    // dbg!(&series);

    let r = new.count_distance_step();
    dbg!(&linear);
    dbg!(&r);
}
