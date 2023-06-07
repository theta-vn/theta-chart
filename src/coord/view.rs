use crate::{coord::{Point, Vector, Rec}, get_bit_at, chart::{Draw, Category}};

// use super::TView;

#[derive(Debug, Clone, Default)]

/// Store data and for a chart, which avaiable to generate a svg or canvas.
pub struct View {
    region: Rec,
    position_axes: usize,
    padding: f64,
    category: Category
    
}
impl View {
    pub fn new(width: u64, height: u64, position_axes: usize, padding: u64) -> Self {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut width = width as f64;
        let mut height = height as f64;
        let padding = padding as f64;

        // Top axes
        if get_bit_at(position_axes, 0) {
            height -= padding;
        }
        // Right axes
        if get_bit_at(position_axes, 1) {
            width -= padding;
        }
        // Bottom axes
        if get_bit_at(position_axes, 2) {
            y += padding;
            height -= padding;
        }
        // Left axes
        if get_bit_at(position_axes, 3) {
            x += padding;
            width -= padding;
        }
        let rec = Rec::new(Point::new(x, y), Vector::new(width as f64, height as f64));
        Self {
            region: rec,
            position_axes: position_axes,
            padding: padding as f64,
            category: Category::default()
        }
    }

    pub fn get_position_axes(&self) -> usize {
        self.position_axes
    }

    pub fn get_padding(&self) -> f64 {
        self.padding
    }
}

impl Draw for View {
    fn get_region(&self) -> Rec {
        self.region.clone()
    }

    fn origin(&self) -> Point {
        self.region.get_origin()
    }

    fn width(&self) -> f64 {
        self.region.get_width()
    }

    fn height(&self) -> f64 {
        self.region.get_height()
    }

    fn chart_type(&self) -> Category {
        self.category
    }

}
