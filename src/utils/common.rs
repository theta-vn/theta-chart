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
/// Check bit of input at n
///
/// Mainly for marking the position of the axes.
///
/// Mapping position axes
/// ```quote
///  0b  0       0      1     1
///      |       |      |     |  
///     left  bottom  right  top
/// ```
///
///  # Example
/// ```rust
/// use theta_chart::get_bit_at;
///
/// let position_axes: usize = 0b0011;
///
/// let left = get_bit_at(position_axes, 0);
/// assert_eq!(true, left);
///
/// let top = get_bit_at(position_axes, 3);
/// assert_eq!(false, top);
/// ```
pub fn get_bit_at(input: usize, n: u8) -> bool {
    if n < 32 {
        input & (1 << n) != 0
    } else {
        false
    }
}
