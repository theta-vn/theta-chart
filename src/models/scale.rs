// use crate::{Number, ScaleLinear};

pub trait ScaleLinear {
    fn series(&self) -> Vec<f64>;
    fn is_float(&self) -> bool;
}

pub trait ScaleCategory {
    fn labels(&self) -> Vec<String>;    
}


// #[derive(Debug, Clone)]
// pub struct Scale<T> {
//     pub data: T
// }

// impl<T> Scale<T>
//     where T: Clone
// {
//     pub fn new(data: T) -> Scale<T> {
//         Scale{
//             data
//         }
//     }
// }

// // impl Scale<Number> {
// //     pub fn linear(&self) -> Vec<Number> {
// //         self.data
// //     }
// // }

// // impl<T> ScaleLinear for Scale<T>
// // where Number: T
// // {
// //     fn series(&self) -> Vec<T> {
// //         self.data
// //     }
// // }
