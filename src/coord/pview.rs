use crate::{
    coord::{Point, Rec, Vector},
    utils::cal_step::min_vec,
};

use super::Circle;
const RADIUS: f64 = 0.9;

#[derive(Debug, Clone, Default)]

/// Store data and for a chart, which avaiable to generate a svg or canvas.
pub struct PView {
    // Width and heigth of all char
    vector: Vector,
    // Region of char not include axes
    region_chart: Circle,
    region_label: Rec,
    // Position of axes (use function get_bit_at to calculate)
    position_label: usize,
    // category: Category,
    margin: f64,
}
impl PView {
    pub fn new(
        width: u64,
        height: u64,
        position_label: usize,
        len_label: u64,
        margin: u64,
    ) -> Self {
        let vector = Vector::new(width as f64, height as f64);
        let margin = margin as f64;
        let width = width as f64 - 2. * margin;
        let height = height as f64 - 2. * margin;
        let len_label = len_label as f64;

        let mut origin_label = Point::default();
        let mut vector_label = Vector::default();
        let mut origin_chart = Point::default();
        let mut radius = 0.;

        match position_label {
            // Label top
            0 => {
                origin_label = Point::new(0., 0.);
                vector_label = Vector::new(width, len_label);
                let width_chart = width;
                let height_chart = height - len_label;
                radius = min_vec(&vec![width_chart, height_chart]) / 2. * RADIUS;
                origin_chart = Point::new(width / 2., height_chart / 2. + len_label);
            }
            // Label right
            1 => {
                origin_label = Point::new(width - len_label, 0.);
                vector_label = Vector::new(len_label, height);
                let width_chart = width - len_label;
                let height_chart = height;
                radius = min_vec(&vec![width_chart, height_chart]) / 2. * RADIUS;
                origin_chart = Point::new((width - len_label) / 2., height / 2.);
            }
            // Label bottom
            2 => {
                origin_label = Point::new(0., height - len_label);
                vector_label = Vector::new(width, len_label);
                let width_chart = width;
                let height_chart = height - len_label;
                radius = min_vec(&vec![width_chart, height_chart]) / 2. * RADIUS;
                origin_chart = Point::new(width / 2., height_chart / 2.);
            }
            // Label left
            3 => {
                // Chart
                origin_label = Point::new(0., 0.);
                vector_label = Vector::new(len_label, height);
                let width_chart = width - len_label;
                let height_chart = height;
                radius = min_vec(&vec![width, height_chart]) / 2. * RADIUS;
                origin_chart = Point::new((width_chart) / 2. + len_label, height / 2.);
            }
            _ => (),
        }

        Self {
            vector,
            region_chart: Circle::new(origin_chart, radius),
            region_label: Rec::new(origin_label, vector_label),
            position_label,
            margin,
        }
    }

    pub fn get_position_label(&self) -> usize {
        self.position_label
    }

    pub fn get_circle_chart(&self) -> Circle {
        self.region_chart.clone()
    }

    pub fn get_rec_label(&self) -> Rec {
        self.region_label.clone()
    }

    pub fn get_vector(&self) -> Vector {
        self.vector.clone()
    }

    // pub fn get_radius_chart(&self) -> Vector {
    //     self.region_chart.get_vector()
    // }

    pub fn get_margin(&self) -> f64 {
        self.margin
    }
}
