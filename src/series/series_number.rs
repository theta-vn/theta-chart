use crate::{chart::*, utils::cal_step::*};

// use super::ScaleLinear;

#[derive(Debug, Clone, Default)]
/// A series of numbers represented on a chart
pub struct SNumber {
    series: Vec<f64>,
    is_float: bool,
    domain: (f64, f64),
    stick: usize,
}

impl SNumber {
    pub fn new(series: Vec<f64>) -> Self {
        let domain = min_max_vec(&series);
        SNumber {
            series,
            is_float: true,
            domain,
            stick: 0,
        }
    }

    pub fn set_stick(&self, stick: usize) -> Self {
        Self {
            series: self.series.clone(),
            is_float: self.is_float,
            domain: self.domain,
            stick: stick,
        }
    }

    pub fn set_domain(&self, min: f64, max: f64) -> Self {
        Self {
            series: self.series.clone(),
            is_float: self.is_float,
            domain: (min, max),
            stick: self.stick,
        }
    }
}

impl From<Vec<i64>> for SNumber {
    fn from(value: Vec<i64>) -> Self {
        let mut series: Vec<f64> = vec![];
        for i in value {
            series.push(i as f64)
        }
        let domain = min_max_vec(&series);
        Self {
            series,
            is_float: false,
            domain,
            stick: 0,
        }
    }
}

impl ScaleNumber for SNumber {
    fn series(&self) -> Vec<f64> {
        self.series.clone()
    }

    fn is_float(&self) -> bool {
        self.is_float
    }

    fn domain(&self) -> (f64, f64) {
        self.domain
    }

    fn count_distance_step(&self) -> (usize, f64, usize) {
        let (min, max) = self.domain();
        let count_distance = if self.stick == 0 { 10 } else { self.stick - 1 };

        if min >= 0. && max >= 0. {
            let mut step = max / count_distance as f64;
            step = CalStep::new(step).cal_scale();
            ((max / step as f64).round() as usize, step, 0)
        } else if min < 0. && max < 0. {
            let mut step = min / count_distance as f64;
            step = CalStep::new(step).cal_scale();
            (0, step, (min.abs() / step as f64).round() as usize)
        } else {
            let mut step = (max - min) / count_distance as f64;
            step = CalStep::new(step).cal_scale();
            (
                (max / step as f64).round() as usize,
                step,
                (min.abs() / step as f64).round() as usize,
            )
        }
    }
}

impl ScaleType for SNumber {
    fn scale_type(&self) -> String {
        "ScaleNumber".to_string()
    }
}
