use theta_chart::*;

#[test]
fn create_datalinear_i64() {
    let i1 = Number::I64(1);
    let i2 = Number::I64(2);
    let new = SLinear::new(vec![i1, i2]);    
    assert_eq!("I64".to_string(), new.type_number() );
}

#[test]
fn create_datalinear_f64() {
    let f1 = Number::F64(1.0);
    let f2 = Number::I64(2);
    let new = SLinear::new(vec![f1, f2]);
    assert_eq!("F64".to_string(), new.type_number() );
}

