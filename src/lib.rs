mod utils;
pub use self::utils::common::*;

/// Color definition and processing module.
pub mod color;

/// Module contains the defined types of data set and the implementation of the defined traits.
pub mod series;

/// Module abstracts the object in descartes coordinate system.
pub mod coord;

// The data processing module, which implements algorithms to mapping the series data to descartes coordinate system.
//

/// Module contains predefined trait, which need implements to mapping the data set to descartes coordinate system.
pub mod chart;
// /// Module abstract the geometric object in descartes coordinate system.
// pub mod shape;
// // pub use self::data::*;

// mod axes;
// pub use self::axes::*;

// mod algebra;
// pub use self::algebra::*;

// mod shape;
// pub use self::shape::*;
