use theta_chart::*;

#[test]
fn create_catersian_ll() {
   
    let linear = Linear::new(vec![1.0, 2.0]);

    let category = Category::new(vec!["A".to_string(), "B".to_string()]);

    // let chart = Cartesian::new(linear, category);
    // dbg!(&scale_x);
    // dbg!(&scale_y);
    // let char_c = Cartesian::<Scale::<Linear>, Scale::<Linear>>{ data_x: scale_x, data_y: scale_y};
    // dbg!(chart);


    let chartt = <Cartesian<Linear, Category> as Descaries>::newt(linear, category);
    // let a = chartt.new();
    dbg!(chartt);

    // let chartt: Cartesian<Linear, Category> = Descaries::newt(linear, category);



    trait Num {
        fn from_i32(n: i32) -> Self;
    }
    
    impl Num for f64 {
        fn from_i32(n: i32) -> f64 { n as f64 }
    }
    
    // These 4 are all equivalent in this case.
    let _: f64 = Num::from_i32(42);
    let _: f64 = <_ as Num>::from_i32(42);
    let _: f64 = <f64 as Num>::from_i32(42);
    let _: f64 = f64::from_i32(42);
}


// // #[test]
// // fn create_catersian_lc() {
// //     let f1 = Number::F64(1.0);
// //     let f2 = Number::I64(2);
// //     let data_line = Linear::new(vec![f1, f2]);

// //     let data_category = Category::new(vec!["A".to_string(), "B".to_string()]);

// //     let scale_x = Scale::<Linear>{data: data_line };
// //     let scale_y = Scale { data: data_category };
// //     dbg!(&scale_x);
// //     dbg!(&scale_y);
// //     let char_c = Cartesian{ data_x: scale_x, data_y: scale_y};
// //     dbg!(char_c);

// // }
