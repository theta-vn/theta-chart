mod series_number;
use crate::chart::{ScaleLabel, ScaleNumber, ScaleTime};
use crate::coord::{Axes, Stick};

pub use self::series_number::SNumber;

mod series_label;
pub use self::series_label::SLabel;

mod series_time;
pub use self::series_time::STime;

#[derive(Debug, Clone)]
pub enum Series {
    Number(SNumber),
    Label(SLabel),
    Time(STime),
}
// For Number
impl From<Vec<f64>> for Series {
    fn from(value: Vec<f64>) -> Self {
        let n = SNumber::new(value);
        Self::Number(n)
    }
}

// For Label
impl From<Vec<&str>> for Series {
    fn from(value: Vec<&str>) -> Self {
        let l = SLabel::from(value);
        Self::Label(l)
    }
}

impl Series {
    pub fn gen_axes(&self) -> Axes {
        match self {
            Series::Number(s) => s.gen_axes(),
            Series::Label(l) => l.gen_axes(),
            Series::Time(t) => t.gen_axes(),
        }
    }

    pub fn get_count(&self) -> usize {
        match self {
            Series::Number(s) => s.series().len(),
            Series::Label(l) => l.labels().len(),
            Series::Time(t) => t.series().len(),
        }
    }

    pub fn to_stick(&self) -> Vec<Stick> {
        match self {
            Series::Number(s) => s.to_stick(),
            Series::Label(l) => l.to_stick(),
            Series::Time(t) => t.to_stick(),
        }
    }

    pub fn scale(&self, value: f64) -> f64 {
        match self {
            Series::Number(s) => s.scale(value),
            Series::Label(l) => l.scale(value),
            Series::Time(_t) => 1.,
        }
    }
}
