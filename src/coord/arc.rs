use crate::{
    coord::{Point, Vector},
    TAU,
};

#[derive(Debug, Default, Clone)]
pub struct Arc {
    pub origin: Point,
    pub begin: Vector,
    pub end: Vector,
    pub sweep: bool,
    pub large: bool,
}

impl Arc {
    pub fn new(origin: Point, begin: Vector, end: Vector, large: bool) -> Self {
        Self {
            origin,
            begin,
            end,
            sweep: true,
            large,
        }
    }

    pub fn new_polar(origin: Point, begin: Vector, tau: f64) -> Self {
        let end = begin.clone().az_rotate_tau(tau);
        let large = tau.abs() >= TAU / 2.0;
        Self {
            origin,
            begin,
            end,
            sweep: true,
            large,
        }
    }

    pub fn delta_xy(&self) -> (f64, f64) {
        let end_point = self.end.to_point();
        let begin_point = self.begin.to_point();
        (
            end_point.get_x() - begin_point.get_x(),
            end_point.get_y() - begin_point.get_y(),
        )
    }

    // TODO: cfg feature SVG
    pub fn gen_path(&self, radius: f64) -> String {
        // let radius = self.begin.module();
        let (dx, dy) = self.delta_xy();
        format!(
            "M 0,0 l {},{} a{},{}  0 {},1 {},{} Z",
            self.begin.get_x() * radius,
            self.begin.get_y() * radius,
            radius,
            radius,
            self.large as i32,
            dx * radius,
            dy * radius
        )
    }

    // // TODO: cfg feature SVG
    // pub fn gen_path(&self, radius: f64) -> String {
    //     // let radius = self.begin.module();
    //     let (dx, dy) = self.delta_xy();
    //     format!(
    //         "M{},{} l{},{} a{},{}  0 {},1 {},{} Z",
    //         self.origin.get_x(),
    //         self.origin.get_y(),
    //         self.begin.get_x(),
    //         self.begin.get_y(),
    //         radius,
    //         radius,
    //         self.large as i32,
    //         dx,
    //         dy
    //     )
    // }
}
