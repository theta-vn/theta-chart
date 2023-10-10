use theta_chart::{self, delaunator::triangulate, coord::Point};

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

