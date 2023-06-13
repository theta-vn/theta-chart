use std::vec;
use chrono::{NaiveDateTime, NaiveDate, ParseError, NaiveTime, Duration};
use crate::{chart::*};


#[derive(Debug, Clone)]
/// A series of numbers represented on a chart
pub struct STime {
    series: Vec<NaiveDateTime>,
    format: String,
    dirty: bool,
    unit: String
}

impl Default for STime {
    fn default() -> Self {
        Self { series: Default::default(), format: "%Y-%m-%d %H:%M:%S".to_string(), dirty: false, unit: "full".to_string() }
    }
}

impl STime {
    pub fn new(series: Vec<NaiveDateTime>) -> Self {
        // let domain = min_max_vec(&series);
        STime {
            series,
            format: "%Y-%m-%d %H:%M:%S".to_string(),
            dirty: false,
            unit: "full".to_string()
        }
    }

    pub fn set_format(&self, format: &str) -> Self {
        STime {
            series: self.series.clone(),
            format: format.to_string(),
            dirty: self.dirty,
            unit: "full".to_string()
        }
    }

    pub fn set_data(&self, series: Vec<NaiveDateTime>) -> Self {
        STime {
            series: series,
            format: self.format.clone(),
            dirty: false,
            unit: "full".to_string()

        }
    }

    pub fn get_unit(&self) -> String {
        self.unit.clone()
    }

    pub fn get_format(&self) -> &str {
        match self.unit.as_str() {            
            "date" => "%Y-%m-%d",
            "month" => "%Y-%m",
            "year" => "%Y",
            "time" => "%H:%M:%S",
            "hour" => "%H:%M:%S",
            _ => "",
        }
        
    }
//     // pub fn set_domain(&self, min: f64, max: f64) -> Self {
//     //     Self {
//     //         series: self.series.clone(),
//     //         is_float: self.is_float,
//     //         domain: (min, max),
//     //         stick: self.stick,
//     //     }
//     // }
}



impl From<(Vec<&str>, &str, &str)> for STime {
    fn from(value: (Vec<&str>, &str, &str)) -> Self {
        let vec = value.0;
        let format = value.1.to_string();
        let mut series: Vec<NaiveDateTime> = vec![];
        let mut dirty = false;
        let unit = value.2.to_string();
        for i in 0..vec.len() {
            dbg!(vec[i], value.1, value.2);
            let rndt = ndt_parse_from_str(vec[i], value.1, value.2);
            dbg!(&rndt);
            match rndt {
                Ok(ndt) => series.push(ndt),
                Err(_) => dirty = true
            }
        }
        STime {
            series,
            format,
            dirty,
            unit
        }
    }
}

fn ndt_parse_from_str(str: &str, format: &str, get: &str) -> Result<NaiveDateTime, ParseError> {
    match get {
        "full" => NaiveDateTime::parse_from_str(str, format),
        "date" => {
            let date = NaiveDate::parse_from_str(str, format);
            match date {
                Ok(d) => {
                    let time = NaiveTime::default();
                    Ok(NaiveDateTime::new(d, time))
                
                },
                Err(e) => Err(e),
            }
            
        },
        "year" => {
            
            let year = format!("{}-01-01", str);            
            dbg!(&year);
            let a = ndt_parse_from_str(year.as_str(), "%Y-%m-%d", "date");
            dbg!(&a);
            a

        },

        _ => {
            NaiveDateTime::parse_from_str(str, format)
        }
    }

    
}


// impl From<Vec<u64>> for SNumber {
//     fn from(value: Vec<u64>) -> Self {
//         let mut series: Vec<f64> = vec![];
//         for i in value {
//             series.push(i as f64)
//         }
//         let domain = min_max_vec(&series);
//         Self {
//             series,
//             is_float: false,
//             domain,
//             stick: 0,
//         }
//     }
// }
impl ScaleTime for STime {
    fn series(&self) -> Vec<NaiveDateTime> {
        self.series.clone()
    }

    fn domain(&self) -> (NaiveDateTime, NaiveDateTime) {
        // TODO: not impl
        (NaiveDateTime::default(), NaiveDateTime::default())
    }

    fn count_distance_step(&self) -> (usize, chrono::Duration, usize) {
        // TODO: not impl
        
        (1,Duration::days(1), 1)
    }

    fn get_interval(&self, len: f64) -> f64 {
        // TODO: not impl
        len / 3.
    }
}

impl ScaleType for STime {
    fn scale_type(&self) -> String {
        "ScaleTime".to_string()
    }
}
