use theta_chart::*;

#[test]
fn create_catersian_ll() {
    let linear = Linear::new(vec![1.0, 2.0]);

    let category = Category::from(vec!["A", "B"]);

    let chart_lc = Cartesian::<Linear, Category>::default()
        .set_ax(linear.clone())
        .set_ay(category.clone());

    dbg!(&chart_lc);

    let chart_ll = Cartesian::<Linear, Linear>::default()
        .set_ax(linear.clone())
        .set_ay(linear.clone());

    dbg!(chart_ll);

    let chart_cc: Cartesian<Category, Category> = Cartesian::default()
        .set_ax(category.clone())
        .set_ay(category.clone());

    dbg!(&chart_cc);

    let series = linear.series();
    dbg!(&series);
    let labels = category.labels();
    dbg!(&labels);

    let colors = category.colors();
    dbg!(&colors);
    let hex_color = colors[0].to_string_hex();
    dbg!(&hex_color);
}
