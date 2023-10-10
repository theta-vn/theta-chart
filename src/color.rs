use palette::{rgb::Rgb, FromColor, IntoColor, Lch, ShiftHue, Srgb};
use std::str::FromStr;

pub const COLOR_PRIMARY_U32: u32 = 0x005bbe;
pub const SHIFT_HUE: f32 = 70.;

#[derive(Debug, Clone)]
pub struct Color(Rgb);

impl Default for Color {
    fn default() -> Self {
        Color(Srgb::from(COLOR_PRIMARY_U32).into_format())
    }
}

impl From<&str> for Color {
    fn from(hex: &str) -> Self {
        match Rgb::from_str(hex) {
            Ok(color) => Self(color.into_format()),
            Err(_) => Self(Srgb::from(COLOR_PRIMARY_U32).into_format()),
        }
    }
}

impl Color {
    // For web
    pub fn to_string_hex(&self) -> String {
        let com = self.0.into_format::<u8>().into_components();
        format!("#{:02X?}{:02X?}{:02X?}", com.0, com.1, com.2)
    }

    pub fn shift_hue(&self) -> Color {
        let lch_color: Lch = self.0.into_color();
        Color(Srgb::from_color(lch_color.shift_hue(SHIFT_HUE)))
    }

    pub fn shift_hue_degrees_index(&self, degrees: f32, index: usize) -> Color {
        let lch_color: Lch = self.0.into_color();
        let degrees = degrees * index as f32;
        Color(Srgb::from_color(lch_color.shift_hue(degrees)))
    }
}
