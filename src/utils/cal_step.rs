#[derive(Debug)]
pub(crate) struct CalStep {
    origin: f64,
    focus: i64,
    multi_10: i64,
}

impl CalStep {
    pub fn new(num: f64) -> Self {
        let cal = CalStep {
            origin: num.abs(),
            focus: 0,
            multi_10: 0,
        };
        cal.log10()
    }

    fn log10(&self) -> Self {
        let (num, log10) = if self.origin < 1.0 {
            cal_log10f(self.origin, self.multi_10)
        } else {
            cal_log10i(self.origin, self.multi_10)
        };

        Self {
            origin: self.origin,
            focus: num as i64,
            multi_10: log10,
        }
    }

    pub fn cal_scale(&self) -> f64 {
        let scale = cal_scale(self.focus);
        let multi_10 = self.multi_10;
        let (scale_r, _multi_10) = cal_multi(scale as f64, multi_10);
        scale_r
    }
}

fn cal_multi(num: f64, multi: i64) -> (f64, i64) {
    let mut num = num;
    let mut multi = multi;
    if multi == 0 {
        (num, multi)
    } else if multi < 0 {
        num /= 10.;
        multi += 1;
        cal_multi(num, multi)
    } else {
        num *= 10.;
        multi -= 1;
        cal_multi(num, multi)
    }
}

pub fn cal_scale(num: i64) -> i64 {
    let string = format!("{:#0b}", num);
    dbg!(&string);
    let count_bit = string.len() - 2;
    dbg!(count_bit);
    if count_bit > 6 {
        100
    } else if count_bit > 5 {
        50
    } else if count_bit > 4 {
        if num > 20 {
            25
        } else {
            20
        }
    } else if count_bit > 3 {
        if num > 10 {
            20
        } else {
            10
        }
    } else if count_bit == 3 {
        5
    } else {
        2
    }
}

fn cal_log10f(num: f64, log10: i64) -> (f64, i64) {
    let mut num_int = num;
    let mut log10_int = log10;

    if 10. <= num_int && num_int < 100. {
        (num_int, log10_int)
    } else if num_int <= 10. {
        log10_int -= 1;
        num_int *= 10.;
        cal_log10f(num_int, log10_int)
    } else {
        log10_int += 1;
        num_int /= 10.;
        cal_log10f(num_int, log10_int)
    }
}

fn cal_log10i(num: f64, log10: i64) -> (f64, i64) {
    let mut num_int = num;
    let mut log10_int = log10;

    if 1. <= num_int && num_int < 100. {
        (num_int, log10_int)
    } else if num_int <= 1. {
        log10_int -= 1;
        num_int *= 10.;
        cal_log10i(num_int, log10_int)
    } else {
        log10_int += 1;
        num_int /= 10.;
        cal_log10i(num_int, log10_int)
    }
}
