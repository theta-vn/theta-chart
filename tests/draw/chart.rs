use theta_chart::{coord::*, series::*};

#[test]
fn draw_new() {
    let linear = SNumber::new(vec![1.0, 2.0]);
    let category = SLabel::from(vec!["A", "B"]);
    let view = CView::new(800, 600, 0b1100, 30, 10);

    let chart = Chart::default()
        .set_ax(linear.clone())
        .set_ay(category.clone())
        .set_view(view);

    dbg!(&chart);

    // assert_relative_eq!(Point::new(30., 30.), view.origin());
    // assert_relative_eq!(Vector::new(770., 570.), view.get_region().get_vector());
    // assert_relative_eq!(arc_oxy.delta_xy().1, arc_polar.delta_xy().1);
    // dbg!(arc_polar, arc_oxy);
}
