use std::vec;

use crate::{chart::*, coord::*, utils::cal_step::*, TAU};


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


impl From<Vec<u64>> for SNumber {
    fn from(value: Vec<u64>) -> Self {
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
            ((max / step as f64).ceil() as usize, step, 0)
        } else if min < 0. && max < 0. {
            let mut step = min / count_distance as f64;
            step = CalStep::new(step).cal_scale();
            (0, step, (min.abs() / step as f64).ceil() as usize)
        } else {
            let mut step = (max - min) / count_distance as f64;
            step = CalStep::new(step).cal_scale();
            (
                (max / step as f64).ceil() as usize,
                step,
                (min.abs() / step as f64).ceil() as usize,
            )
        }
    }


    fn to_percent(&self) -> Vec<f64> {
        let total: f64 = self.series.iter().sum();
        self.series.clone().into_iter().map(|f| f / total).collect()
    }


    
    fn gen_pie(&self, origin: Point, radius: f64) -> Vec<Arc> {
        let series = self.series.clone();
        let total: f64 = series.iter().sum();
        let percent: Vec<f64> = series.clone().into_iter().map(|f| f / total).collect();
        let mut vector_begin = Vector::new(0., radius);
        let mut vec_arc: Vec<Arc> = vec![];
        for p in percent {
            let arc = Arc::new_polar(origin.clone(), vector_begin.clone(), p * TAU);
            vector_begin = arc.end.clone();
            vec_arc.push(arc);
        }
        vec_arc
    }

    fn get_interval(&self, len: f64) -> f64 {
        let (distance_up, _step, distence_down) = self.count_distance_step();
        len / ((distance_up + distence_down)as f64) 
    }

    fn gen_sticks_label_step(&self) -> (Vec<String>, f64) {
        let (distance_up, step, distance_down) = self.count_distance_step();
        let mut vec_string: Vec<String> = vec![];        
        let (_, precision) = count_precision(step.clone(), 0);
        dbg!(precision);
        for index in 0..(distance_up + distance_down + 1) {
            vec_string.push(format!("{:.prec$}", index as f64 * step, prec=precision));                  
          
        };
        (vec_string, step)
    }
}

fn count_precision(mut number: f64, mut count: usize) -> (f64, usize) {
    
    let floor = number - number.floor();
    if floor == 0. {
        return (number, count)
    } else {
        
         number *= 10.;
         count += 1;
        count_precision(number, count)
    }
}

impl ScaleType for SNumber {
    fn scale_type(&self) -> String {
        "ScaleNumber".to_string()
    }
}
