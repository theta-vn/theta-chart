pub struct AxesLinear {
    origin: f64, // Vi tri goc cua truc pixel
    position: usize, // 0: Top,1: Right ,2: Bottom,3: Left
    range: (f64, f64), // Vung gia tri the hien ra
    // domain: (f64, f64), // Mien gia tri cua truc tuong un
    ratio: f64 // Ti le giua gia trị thuc va vung the hien pixel/value
}

// impl AxesLinear {
//     pub fn new() -> Self {
//         Self { origin: (), position: (), range: (),  ratio: () }
//     }
// }