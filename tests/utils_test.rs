use theta_chart::{self, coord::Point, delaunator::*, series::{SNumber, Series}};

#[test]
fn delaunater() {
    let points = vec![
        Point::new(0., 0.),
        Point::new(1., 0.),
        Point::new(1., 1.),
        Point::new(0., 1.),
    ];

    let result = triangulate(&points);

    println!("{:?}", result.triangles); // [0, 2, 1, 0, 3, 2]
}

#[test]
fn delaunay() {

    let xvf64: Vec<f64> = vec![1., 3., 5., 9., 15., 27.,];
    let xlinear = SNumber::new(xvf64).set_range(0., 28.);    
    let xseires = Series::Number(xlinear);
    // dbg!(&xseires);


    let yvf64: Vec<f64> = vec![2., 4., 5., 19., 12., 24.,];
    let ylinear = SNumber::new(yvf64).set_range(1., 20.);    
    let yseires = Series::Number(ylinear);
    // dbg!(&yseires);

    let vdelaunay = triangle(xseires, yseires);
    dbg!(vdelaunay);
    // let points = vec![
    //     Point::new(0., 0.),
    //     Point::new(1., 0.),
    //     Point::new(1., 1.),
    //     Point::new(0., 1.),
    // ];

    // let result = triangulate(&points);

    // println!("{:?}", result.triangles); // [0, 2, 1, 0, 3, 2]
}
