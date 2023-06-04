use crate::ScaleLinear;

#[derive(Debug, Clone)]
pub struct Linear
{
    series: Vec<f64>,
    is_float: bool,
}

impl Linear {
    pub const fn new(series: Vec<f64>) -> Self {
        Linear { series, is_float: true  }
    }
}

impl From<Vec<i64>> for Linear {
    fn from(value: Vec<i64>) -> Self {
        let mut series: Vec<f64> = vec![];
        for i in value {
            series.push(i as f64)
        }    
        Self{series, is_float: false }
    }
}

impl ScaleLinear for Linear {
    fn series(&self) -> Vec<f64> {
        self.series.clone()
    }

    fn is_float(&self) -> bool {
        self.is_float
    }
}
