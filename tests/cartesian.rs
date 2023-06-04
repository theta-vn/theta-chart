use theta_chart::*;

#[test]
fn create_catersian_ll() {
    let f1 = Number::F64(1.0);
    let f2 = Number::I64(2);
    let data_line = SLinear::new(vec![f1, f2]);

    let scale_x = SScale::<SLinear>{data: data_line };
    let scale_y = scale_x.clone();
    dbg!(&scale_x);
    dbg!(&scale_y);
    let char_c = SCartesian::<SScale::<SLinear>, SScale::<SLinear>>{ data_x: scale_x, data_y: scale_y};
    dbg!(char_c);

}

#[test]
fn create_catersian_lc() {
    let f1 = Number::F64(1.0);
    let f2 = Number::I64(2);
    let data_line = SLinear::new(vec![f1, f2]);

    let data_category = SCategory::new(vec!["A".to_string(), "B".to_string()]);

    let scale_x = SScale::<SLinear>{data: data_line };
    let scale_y = SScale { data: data_category };
    dbg!(&scale_x);
    dbg!(&scale_y);
    let char_c = SCartesian{ data_x: scale_x, data_y: scale_y};
    dbg!(char_c);

}
