use crate::coord::{Point, Rec, Vector};

#[derive(Debug, Clone, Default)]

/// Store data and for a chart, which avaiable to generate a svg or canvas.
pub struct CView {
    // Width and heigth of all char
    vector: Vector,
    // Region of char not include axes
    region_chart: Rec,
    region_x_axis: Rec,
    region_y_axis: Rec,
    // Position of axes (use function get_bit_at to calculate)
    position_origin: usize,
    // category: Category,
    margin: f64,
}
impl CView {
    pub fn new(
        width: u64,
        height: u64,
        position_origin: usize,
        height_x_axis: u64,
        width_y_axis: u64,
        margin: u64,
    ) -> Self {
        // let padding = padding as f64;
        let height_x_axis = height_x_axis as f64;
        let width_y_axis = width_y_axis as f64;
        let margin = margin as f64;
        let vector = Vector::new(width as f64, height as f64);

        let width = width as f64 - 2. * margin;
        let height = height as f64 - 2. * margin;

        let mut origin = Point::default();
        let mut vec_chart = Vector::new(width, height);

        let mut vec_x_axis = Vector::default();
        // let mut ori_x_axis = Point::default();

        let mut vec_y_axis = Vector::default();
        // let mut ori_y_axis = Point::default();

        let position_origin = position_origin;

        match position_origin {
            // Origin top left
            0 => {
                // Chart
                origin = Point::new(width_y_axis, height_x_axis);
                vec_chart = Vector::new(width - width_y_axis, height - height_x_axis);
                vec_x_axis = Vector::new(width - width_y_axis, -height_x_axis);
                vec_y_axis = Vector::new(-width_y_axis, height - height_x_axis);
            }
            // Origin top right
            1 => {
                // Chart
                origin = Point::new(width - width_y_axis, height_x_axis);
                vec_chart = Vector::new(-(width - width_y_axis), height - height_x_axis);
                vec_x_axis = Vector::new(-(width - width_y_axis), -height_x_axis);
                vec_y_axis = Vector::new(width_y_axis, height - height_x_axis);
            }
            // Origin bottom right
            2 => {
                // Chart
                origin = Point::new(width - width_y_axis, height - height_x_axis);
                vec_chart = Vector::new(-(width - width_y_axis), -(height - height_x_axis));
                vec_x_axis = Vector::new(-(width - width_y_axis), height_x_axis);
                vec_y_axis = Vector::new(width_y_axis, -(height - height_x_axis));
            }
            3 => {
                // Chart
                origin = Point::new(width_y_axis, height - height_x_axis);
                vec_chart = Vector::new(width - width_y_axis, -(height - height_x_axis));
                vec_x_axis = Vector::new(width - width_y_axis, height_x_axis);
                vec_y_axis = Vector::new(-width_y_axis, -(height - height_x_axis));
            }
            _ => (),
        }

        let rec_chart = Rec::new(origin.clone(), vec_chart);
        let rec_x_axis = Rec::new(origin.clone(), vec_x_axis);
        let rec_y_axis = Rec::new(origin, vec_y_axis);
        Self {
            vector,
            region_chart: rec_chart,
            position_origin: position_origin,
            region_x_axis: rec_x_axis,
            region_y_axis: rec_y_axis,
            margin: margin as f64,
        }
    }

    pub fn get_position_origin(&self) -> usize {
        self.position_origin
    }

    pub fn get_rec_chart(&self) -> Rec {
        self.region_chart.clone()
    }

    pub fn get_rec_x_axis(&self) -> Rec {
        self.region_x_axis.clone()
    }

    pub fn get_rec_y_axis(&self) -> Rec {
        self.region_y_axis.clone()
    }

    pub fn get_vector(&self) -> Vector {
        self.vector.clone()
    }

    // pub fn get_vector_chart(&self) -> Vector {
    //     self.region_chart.get_vector()
    // }

    pub fn get_margin(&self) -> f64 {
        self.margin
    }
}
