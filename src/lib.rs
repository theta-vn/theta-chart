//! # Calculations for projects about drawing svg
//!
//! `theta_chart` is a collection of utilities to make performing certain
//! calculations more convenient.

pub(crate) mod utils;

/// Color for drawing.
pub mod color;
// pub use self::color::{Color};

mod models;
pub use self::models::*;

// mod axes;
// pub use self::axes::*;

mod algebra;
pub use self::algebra::*;

mod shape;
pub use self::shape::*;
