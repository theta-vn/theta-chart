mod series_number;
pub use self::series_number::SNumber;
use crate::chart::{ScaleLabel, ScaleNumber, ScaleTime};
use crate::coord::{Axes, Stick};
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

// For Number
impl From<Vec<i64>> for Series {
    fn from(value: Vec<i64>) -> Self {
        let n = SNumber::from(value);
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

// For Time
impl From<(Vec<&str>, &str, &str)> for Series {
    fn from(value: (Vec<&str>, &str, &str)) -> Self {
        let st = STime::from(value);
        Self::Time(st)
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
            Series::Time(_t) => 1. * value,
        }
    }

    pub fn scale_index(&self, label: String) -> usize {
        match self {
            Series::Number(_s) => 1,
            Series::Label(l) => l.scale_index(label),
            Series::Time(_t) => 1,
        }
    }

    pub fn set_range(&self, min: f64, max: f64) -> Self {
        match self {
            Series::Number(s) => Series::Number(s.set_range(min, max)),
            Series::Label(l) => Series::Label(l.clone()),
            Series::Time(t) => Series::Time(t.clone()),
        }
    }

    pub fn series_stick(&self) -> Vec<Stick> {
        match self {
            Series::Number(s) => s.to_stick(),
            Series::Label(_l) => vec![],
            Series::Time(t) => t.to_stick(),
        }
    }

    // pub fn series(&self) -> Vec {
    //     match self {
    //         Series::Number(s) => s.series(),
    //         Series::Label(_l) => vec![],
    //         Series::Time(t) => t.to_stick(),
    //     }
    // }
}
