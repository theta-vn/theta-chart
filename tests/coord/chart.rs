use theta_chart::{coord::*, series::*};

#[test]
fn cartersian_create_ll() {
    // let linear = SNumber::new(vec![1.0, 2.0]);
    let category = SLabel::from(vec!["A", "B"]);
    // let view = CView::new(800, 600, 0b1100, 30, 10);

    let chart_lc = Cartesian::new(Series::from(vec![1.0, 2.0]), Series::Label(category));
    // .set_ax(linear.clone())
    // .set_ay(category.clone())
    // .set_view(view);

    dbg!(&chart_lc);

    // let chart_ll = Chart::<SeriesNumber, SeriesNumber>::default()
    //     .set_ax(linear.clone())
    //     .set_ay(linear.clone());

    // dbg!(chart_ll);

    // let chart_cc: Chart<SeriesLabel, SeriesLabel> = Chart::default()
    //     .set_ax(category.clone())
    //     .set_ay(category.clone());

    // dbg!(&chart_cc);

    // let series = linear.series();
    // dbg!(&series);
    // let labels = category.labels();
    // dbg!(&labels);

    // let colors = category.colors();
    // dbg!(&colors);
    // let hex_color = colors[0].to_string_hex();
    // dbg!(&hex_color);
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
        .add_data(
            Series::from(vec![0., 1.0, 2., 3.]),
            Series::Time(st)
        )        
        .set_view(820, 620, 3, 100, 100, 20);
    
    dbg!(&chart);
    
}

