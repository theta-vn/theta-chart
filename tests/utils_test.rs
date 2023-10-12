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

    let xvf64: Vec<i64> = vec![0, 12, 8, 9, 0, 5, 9, 5];
    let xlinear = SNumber::from(xvf64).set_range(0., 28.);    
    let xseires = Series::Number(xlinear);
    // dbg!(&xseires);


    let yvf64: Vec<i64> = vec![2, 7, 7, 1, 12, 5, 12, 8];
    let ylinear = SNumber::from(yvf64).set_range(1., 20.);    
    let yseires = Series::Number(ylinear);
    // dbg!(&yseires);

    let vdelaunay = triangle(xseires, yseires);
    dbg!(&vdelaunay);
    dbg!(&vdelaunay.triangles.len());
    dbg!(&vdelaunay.halfedges.len());


    // for (pos, e) in vdelaunay.halfedges.iter().enumerate() {
    //     dbg!(format!("TRIANGLES:{:#?}-HALFEDFES:{:#?}", vdelaunay.triangles[pos], e));
    // }
    // let points = vec![
    //     Point::new(0., 0.),
    //     Point::new(1., 0.),
    //     Point::new(1., 1.),
    //     Point::new(0., 1.),
    // ];

    // let result = triangulate(&points);

    // println!("{:?}", result.triangles); // [0, 2, 1, 0, 3, 2]
}
