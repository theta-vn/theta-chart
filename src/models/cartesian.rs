use crate::{ScaleLinear, ScaleCategory, Category, Linear};

use super::{category, linear};

#[derive(Debug)]
pub struct Cartesian<T, U> {
    dx: T,
    dy: U
}


// Imple generic
impl<T,U> Cartesian<T,U> {
    pub fn new(dx: T, dy: U) -> Self {
        Self { dx, dy }
    }
    
}

pub trait Descaries {
    type A;
    type B;
    
    fn newt(dx: Self::A, dy: Self::B) -> Cartesian<Linear,Category> ;
}

// pub struct Des;
// trait Des {
//     fn from_i32(n: i32) -> Self;
// }

// impl Des for f64 {
//     fn from_i32(n: i32) -> f64 { n as f64 }
// }

// impl<T, U> DoubleDrop<T> for U {
impl<T,U> Descaries for Cartesian<T,U> {
    type A = Linear;
    type B = Category ;
    

    fn newt(dx: Linear, dy: Category) -> Cartesian<Linear,Category> {
        Cartesian{dx, dy}
    }   
}


// impl<T, U> Cartesian<T, U> 
// where T: ScaleCategory,
//     U:ScaleLinear
// {    
//     pub fn new_cl(dx: T, dy: U) -> Self {
//         Self { dx, dy }
//     }   
// }

// impl<T, U> Cartesian<T, U> 
// where U: ScaleCategory,
//     T: ScaleLinear
// {    
//     pub fn new_lc(dx: T, dy: U) -> Self {
//         Self { dx, dy }
//     }   
// }

// impl<T> Cartesian<T, T> 
// where T: ScaleLinear
// {
//     pub fn new_ll(dx: T, dy: T) -> Self {
//         Self { dx, dy }
//     }
    
// }

// impl<T> Cartesian<T, T> 
// where T: ScaleCategory
// {
//     pub fn new_cc(dx: T, dy: T) -> Self {
//         Self { dx, dy }
//     }
    
// }
// impl<T, U> From<(T,U)> for Cartesian<T, U>
// where T: ScaleLinear, U: ScaleLinear
// {
//     fn from(data: (T,U)) -> Self {       
//         Self { dx: data.0, dy: data.1 }
//     }
// }


// impl<T, U> From<(T,U, bool)> for Cartesian<T, U>
// where T: ScaleLinear, U: ScaleCategory
// {
//     fn from(data: (T,U, bool)) -> Self {       
//         Self { dx: data.0, dy: data.1 }
//     }
// }

// impl<T, U> From<(T,bool, U)> for Cartesian<T, U>
// where T: ScaleLinear, U: ScaleCategory
// {
//     fn from(data: (T,U, bool)) -> Self {       
//         Self { dx: data.0, dy: data.1 }
//     }
// }


