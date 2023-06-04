use theta_chart::*;

#[test]
fn create_scale_line() {
    let f1 = Number::F64(1.0);
    let f2 = Number::I64(2);
    let data_line = SLinear::new(vec![f1, f2]);

    let scale = SScale::<SLinear>{data: data_line };
    dbg!(scale);
    // let i1 = Number::I64(1);
    // let i2 = Number::I64(2);
    // let new = DataLinear::new(vec![i1, i2]);    
    // assert_eq!("I64".to_string(), new.type_number() );
}
