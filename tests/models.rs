use theta_chart::*;

#[test]
fn create_linear() {
    let vec_f64 = vec![1., 2.];
    let linear_f64 = Linear::new(vec_f64);
    assert_eq!(true, linear_f64.is_float());

    let vec_i64 = vec![1, 2];
    let linear_i64 = Linear::from(vec_i64);
    assert_eq!(false, linear_i64.is_float());
}
