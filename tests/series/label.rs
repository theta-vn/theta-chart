use theta_chart::series::SLabel;

#[test]
fn create_series_label() {
    let labels = SLabel::from(vec!["A", "B", "C"]);
    dbg!(&labels);
}
