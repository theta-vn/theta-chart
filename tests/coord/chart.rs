use theta_chart::{coord::*, series::*};

#[test]
fn c1artersian_create_ll() {
    let linear = SNumber::new(vec![1.0, 2.0]);
    let category = SLabel::from(vec!["A", "B"]);

    let chart_lc = Chart::default()
        .set_ax(linear.clone())
        .set_ay(category.clone());

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
