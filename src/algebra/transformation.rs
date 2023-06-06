
// pub fn rotate( ratio: f64) -> Point {
//     let arc = ratio * TAU;
//     let axisangle = Vector3::z() * arc;
//     let iso = Isometry3::new(Vector3::default(), axisangle);
//     let p = iso * self.value();
//     Point(p)
// }