use theta_chart::{coord::*, series::*};

#[test]
fn cartersian_create_ll() {
    let category = SLabel::from(vec!["A", "B"]);

    let chart_lc = Cartesian::new(Series::from(vec![1.0, 2.0]), Series::Label(category));

    dbg!(&chart_lc);
}

#[test]
fn cartersian_create_error() {
    // let linear = SNumber::new(vec![1.0, 2.0]);
    let chart = Cartesian::new(
        Series::from(vec![0., 1.0, 2., 3., 4.]),
        // Series::from(vec![3.0, 1.0, 5., 8., 7.]),
        Series::from((vec!["1982", "1986", "2010", "2020"], "%Y", "year")),
    )
    .set_view(820, 620, 3, 100, 100, 20);

    dbg!(&chart);
}

#[test]
fn cartersian_group_create_nn() {
    let chart = CartesianGroup::new()
        .add_data(
            Series::from(vec![0., 1.0, 2., 3., 4.]),
            Series::from(vec![3.0, 1.0, 5., 8., 7.]),
        )
        .add_data(
            Series::from(vec![0., 1.0, 2., 3., 4.]),
            Series::from(vec![9.0, 2.0, 5., 4., 7.]),
        )
        .set_view(820, 620, 3, 100, 100, 20);

    dbg!(&chart);
}

#[test]
fn cartersian_group_create_nt() {
    let st = STime::from((vec!["1982", "1986", "2017", "2020"], "%Y", "year"));
    let chart = CartesianGroup::new()
        .add_data(Series::from(vec![0., 1.0, 2., 3.]), Series::Time(st))
        .set_view(820, 620, 3, 100, 100, 20);

    dbg!(&chart);
}

#[test]
fn cartersian_group_create_ln() {
    let chart = CartesianGroup::new()
        .set_view(840, 640, 3, 50, 50, 20)
        .add_data(Series::from(vec![0.7, 1.5]), Series::from(vec!["A", "B"]))
        .add_data(Series::from(vec![0.3, 0.9]), Series::from(vec!["A", "C"]));

    let series_y_group = chart.get_ay_group();

    dbg!(&series_y_group);
}
