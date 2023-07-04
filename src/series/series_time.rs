use crate::{
    chart::*,
    coord::{Axes, Stick},
    utils::cal_step::CalStep,
};
use chrono::{Datelike, Months, NaiveDate, NaiveDateTime, NaiveTime, ParseError};
use std::vec;

#[derive(Debug, Clone)]
/// A series of numbers represented on a chart
pub struct STime {
    series: Vec<NaiveDateTime>,
    format: String,
    dirty: bool,
    unit: String,
}

impl Default for STime {
    fn default() -> Self {
        Self {
            series: Default::default(),
            format: "%Y-%m-%d %H:%M:%S".to_string(),
            dirty: false,
            unit: "full".to_string(),
        }
    }
}

impl STime {
    pub fn new(series: Vec<NaiveDateTime>) -> Self {
        // let domain = min_max_vec(&series);
        STime {
            series,
            format: "%Y-%m-%d %H:%M:%S".to_string(),
            dirty: false,
            unit: "full".to_string(),
        }
    }

    pub fn set_format(&self, format: &str) -> Self {
        STime {
            series: self.series.clone(),
            format: format.to_string(),
            dirty: self.dirty,
            unit: "full".to_string(),
        }
    }

    pub fn set_data(&self, series: Vec<NaiveDateTime>) -> Self {
        STime {
            series: series,
            format: self.format.clone(),
            dirty: false,
            unit: "full".to_string(),
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

    pub fn series(&self) -> Vec<NaiveDateTime> {
        self.series.clone()
    }

    pub fn get_nv(&self, index: usize) -> NaiveDateTime {
        self.series[index]
    }

    pub fn merge(&self, other: STime) -> Self {
        let mut series = self.series.clone();
        series.extend(&other.series);
        Self {
            series: series.clone(),
            format: self.format.clone(),
            dirty: self.dirty,
            unit: self.unit.clone(),
        }
    }
}

impl From<(Vec<&str>, &str, &str)> for STime {
    fn from(value: (Vec<&str>, &str, &str)) -> Self {
        let vec = value.0;
        let format = value.1.to_string();
        let mut series: Vec<NaiveDateTime> = vec![];
        let mut dirty = false;
        let unit = value.2.to_string();
        for i in 0..vec.len() {
            let rndt = ndt_parse_from_str(vec[i], value.1, value.2);
            match rndt {
                Ok(ndt) => series.push(ndt),
                Err(_) => dirty = true,
            }
        }
        STime {
            series,
            format,
            dirty,
            unit,
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
                }
                Err(e) => Err(e),
            }
        }
        "year" => {
            let year = format!("{}-01-01", str);
            ndt_parse_from_str(year.as_str(), "%Y-%m-%d", "date")
        }
        "month" => {
            let month = format!("{}-01", str);
            ndt_parse_from_str(&month.as_str(), "%Y-%m-%d", "date")
        }

        _ => NaiveDateTime::parse_from_str(str, format),
    }
}

impl ScaleTime for STime {
    fn domain(&self) -> (NaiveDateTime, NaiveDateTime) {
        if self.series().len() > 0 {
            let binding = self.series();
            let min = binding.iter().min().unwrap();
            let max = binding.iter().max().unwrap();
            (*min, *max)
        } else {
            (NaiveDateTime::default(), NaiveDateTime::default())
        }
    }

    fn domain_unix(&self) -> (f64, f64) {
        let (min, max) = self.domain();
        match self.unit.as_str() {
            "year" => (min.year() as f64, max.year() as f64),
            "month" => {
                let min = min.year() as f64 * 12. + min.month0() as f64;
                let max = max.year() as f64 * 12. + max.month0() as f64;
                (min, max)
            }
            _ => (0., 0.),
        }
    }

    // fn count_distance_step(&self) -> (f64, f64) {
    //     match self.get_unit().as_str() {
    //         "year" => {
    //             let (min, max) = self.domain_unix();
    //             let dur = max - min;

    //             let mut step = dur as f64 / 5.;
    //             step = CalStep::new(step.ceil()).cal_scale();

    //             (dur as f64 / step, step)
    //         }
    //         // TODO: for month, day, time
    //         _ => (1., 0.),
    //     }
    // }

    // fn scale_intervale(&self, value: NaiveDateTime) -> f64 {
    //     let (min, _max) = self.domain();
    //     let unit = self.get_unit();
    //     match unit.as_str() {
    //         "year" => (value.year() - min.year()) as f64,
    //         _ => (value.year() - min.year()) as f64,
    //     }
    // }

    fn scale(&self, value: NaiveDateTime) -> f64 {
        let unit = self.get_unit();
        match unit.as_str() {
            "year" => {
                let (min, max) = self.domain_unix();
                let range = max - min;

                let diff = value.year() as f64 - min;
                diff / range
            }
            "month" => {
                let (min, max) = self.domain_unix();
                let range = max - min;
                let diff = value.year() as f64 * 12. + value.month0() as f64 - min;
                diff / range
            }
            _ => 1.,
        }
    }

    fn gen_axes(&self) -> Axes {
        let mut style = String::default();
        let mut vec_stick: Vec<Stick> = vec![];
        let mut step = 0.;
        let unit = self.get_unit();

        let (min, max) = self.domain_unix();

        match unit.as_str() {
            "year" => {
                style = "time-year".to_string();
                step = (max - min) / 5.;
                step = CalStep::new(step).cal_scale();

                let first_stick = (min / step).ceil() * step;
                let last_stick = (max / step).floor() * step;
                for value in ((first_stick as i64)..(last_stick as i64 + 1)).step_by(step as usize)
                {
                    let string_value = value.to_string();
                    let nv = ndt_parse_from_str(string_value.as_str(), "%Y", "year").unwrap();
                    let stick = Stick::new(value.to_string(), self.scale(nv));
                    vec_stick.push(stick);
                }
            }
            "month" => {
                style = "time-month".to_string();
                step = cal_scale_time(max - min, "month");

                let first_stick = (min / step).ceil();
                let last_stick = (max / step).floor();

                let interval = last_stick - first_stick;

                let count = (interval) as i64;

                for index in 0..(count + 1) {
                    let value = (first_stick + index as f64) * step;
                    let year = (value / 12.) as i32;
                    let month = ((value / 12. - year as f64) * 12.).round();
                    let nd = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
                    let nd = nd.checked_add_months(Months::new(month as u32)).unwrap();
                    let nv = NaiveDateTime::new(nd, NaiveTime::default());
                    let stick = Stick::new(nv.format("%Y-%m").to_string(), self.scale(nv));
                    vec_stick.push(stick);
                }
            }
            _ => (),
        }

        let sticks = vec_stick
            .into_iter()
            .filter(|stick| stick.value >= -0.0000001 && stick.value <= 1.0000001)
            .collect::<Vec<_>>();

        Axes {
            sticks,
            step,
            style,
        }
    }

    fn to_stick(&self) -> Vec<Stick> {
        let mut vec_stick: Vec<Stick> = vec![];
        let unit = self.get_unit();
        let len = self.series().len();
        match unit.as_str() {
            "year" => {
                for index in 0..len {
                    let nv = self.get_nv(index);
                    let string_value = nv.format(self.get_format()).to_string();
                    let stick = Stick::new(string_value, self.scale(nv));
                    vec_stick.push(stick);
                }
            }
            "month" => {
                for index in 0..len {
                    let nv = self.get_nv(index);
                    let string_value = nv.format(self.get_format()).to_string();
                    let stick = Stick::new(string_value, self.scale(nv));
                    vec_stick.push(stick);
                }
            }
            _ => (),
        }

        vec_stick
    }
}

fn cal_scale_time(num: f64, unit: &str) -> f64 {
    let num = num / 5.;
    match unit {
        "month" => {
            let step_month = CalStep::new(num / 12.).cal_scale() * 12.;
            if step_month > 1. {
                return step_month.round();
            } else {
                step_month
            }
        }
        _ => num,
    }
}
