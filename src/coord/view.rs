use crate::{
    chart::{Category, Draw},
    coord::{Point, Rec, Vector},
    get_bit_at,
    utils::cal_step::min_vec,
};

// use super::TView;

#[derive(Debug, Clone, Default)]

/// Store data and for a chart, which avaiable to generate a svg or canvas.
pub struct CView {
    // Width and heigth of all char
    vector: Vector,
    // Region of char not include axes
    region: Rec,
    // Position of axes (use function get_bit_at to calculate)
    position_axes: usize,
    // intervale to edge outer of axes
    padding: f64,
    category: Category,
    margin: f64,
}
impl CView {
    pub fn new(width: u64, height: u64, position_axes: u64, padding: u64, margin: u64) -> Self {
        let padding = padding as f64;
        let margin = margin as f64;
        let vector = Vector::new(width as f64, height as f64);
        let mut x = 0.;
        let mut y = 0.;
        let mut width = width as f64 - 2. * margin;
        let mut height = height as f64 - 2. * margin;

        let position_axes = position_axes as usize;

        // Top axes
        if get_bit_at(position_axes, 0) {
            y += padding;
            height -= padding;
        }
        // Right axes
        if get_bit_at(position_axes, 1) {
            width -= padding;
        }
        // Bottom axes
        if get_bit_at(position_axes, 2) {
            height -= padding;
        }
        // Left axes
        if get_bit_at(position_axes, 3) {
            x += padding;
            width -= padding;
        }
        let rec = Rec::new(Point::new(x, y), Vector::new(width as f64, height as f64));
        Self {
            vector,
            region: rec,
            position_axes: position_axes,
            padding: padding as f64,
            category: Category::default(),
            margin: margin as f64,
        }
    }

    pub fn get_position_axes(&self) -> usize {
        self.position_axes
    }

    pub fn get_padding(&self) -> f64 {
        self.padding
    }

    pub fn get_vector(&self) -> Vector {
        self.vector.clone()
    }

    // For piechart to get region
    pub fn get_position_axes_first(&self) -> usize {
        let result = 5;
        for p in 0..3 {
            if get_bit_at(self.position_axes, p) {
                return p as usize;
            }
        }
        result
    }

    fn get_region_axes(&self, position: usize) -> Rec {
        let padding = self.padding;
        let inner = &self.get_inner();

        match position {
            0 => Rec::new(Point::new(0., 0.), Vector::new(inner.get_x(), padding)),
            1 => Rec::new(
                Point::new(inner.get_y(), 0.),
                Vector::new(padding, inner.get_y()),
            ),
            2 => Rec::new(
                Point::new(0., inner.get_y()),
                Vector::new(inner.get_x(), padding),
            ),
            3 => Rec::new(Point::new(0., 0.), Vector::new(padding, inner.get_y())),
            _ => Rec::new(Point::default(), Vector::default()),
        }
    }

    pub fn get_region_axes_first(&self) -> Rec {
        let position = self.get_position_axes_first();
        self.get_region_axes(position)
    }
}

impl From<Vec<u64>> for CView {
    fn from(view: Vec<u64>) -> CView {
        CView::new(view[0], view[1], view[2], view[3], view[4])
    }
}

impl Draw for CView {
    fn get_origin(&self) -> Point {
        self.region.get_origin()
    }

    fn get_outer(&self) -> Vector {
        self.get_vector()
    }

    fn get_inner(&self) -> Vector {
        self.region.get_vector()
    }

    // fn get_region(&self) -> Rec {
    //     self.region.clone()
    // }

    // fn origin(&self) -> Point {
    //     self.region.get_origin()
    // }

    // fn width(&self) -> f64 {
    //     self.region.get_width()
    // }

    // fn height(&self) -> f64 {
    //     self.region.get_height()
    // }

    fn chart_type(&self) -> Category {
        self.category
    }

    fn get_center(&self) -> Point {
        let origin = self.get_origin();
        let inner = self.get_inner();
        origin.translate(&inner.multiply(0.5))
    }

    fn get_radius(&self) -> f64 {
        let inner = self.get_inner();
        let diameter = min_vec(&vec![inner.get_x(), inner.get_y()]);
        diameter / 2.
    }

    fn get_margin(&self) -> f64 {
        self.margin
    }
}
