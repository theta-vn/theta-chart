#[derive(Debug)]
pub enum EScale {
    Linear,
    Category
}

// pub trait Scale {
    
// }


#[derive(Debug, Clone)]
pub struct SScale<T: Clone> {
    // pub scale_type: EScale,
    pub data: T
}

// impl<T> Scale<T> {
//     fn new(data: T) -> Self<T> {
//         Scale{

//         }
//     }
// }

// pub trait ScaleLinear<Number> {
//     fn data(&self) -> Vec<Number>;
//     fn domain(&self) -> (Number, Number);
//     fn range(&self) -> (Number, Number);
// }

// pub trait ScaleCategory {
//     fn label(self) -> Vec<String>;
// }