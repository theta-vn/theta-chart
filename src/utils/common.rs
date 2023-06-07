/// The full circle constant (τ)
///
/// Equal to 2π
pub const TAU: f64 = std::f64::consts::TAU;

/// Convert from turn to radian
///
/// 1 turn is equal to 2π (τ) (6.283185307179586)
pub fn turn_to_radian(t: f64) -> f64 {
    t * TAU
}

/// Convert from degree to radian
///
/// 360 degrees is equal to 2π (τ) (6.283185307179586)
pub fn degree_to_radian(d: f64) -> f64 {
    d / 360. * TAU
}
