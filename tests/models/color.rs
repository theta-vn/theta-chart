use theta_chart::{self, color::Color};

#[test]
fn color_form() {
    let color_str = "#ff0000";
    let color = Color::from(color_str);
    assert_eq!(color_str.to_string().to_uppercase(), color.to_string_hex());
}

#[test]
fn color_create() {
    let color = Color::default();
    assert_eq!("#005bbe".to_string().to_uppercase(), color.to_string_hex());
}

#[test]
fn color_shift_hue() {
    let color = Color::default();
    let shift = color.shift_hue();
    dbg!(&color, &shift);
    assert_ne!(color.to_string_hex(), shift.to_string_hex());
}
