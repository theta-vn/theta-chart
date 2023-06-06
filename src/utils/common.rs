pub(crate) fn min_vec(vec: &Vec<f64>) -> f64 {
    vec.iter().copied().fold(f64::NAN, f64::min)
}

pub(crate) fn max_vec(vec: &Vec<f64>) -> f64 {
    vec.iter().copied().fold(f64::NAN, f64::max)
}

pub(crate) fn min_max_vec(vec: &Vec<f64>) -> (f64, f64) {
    (min_vec(vec), max_vec(vec))
}
