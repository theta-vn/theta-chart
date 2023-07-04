use super::CView;
use crate::series::Series;

#[derive(Debug, Clone)]
/// Store data for descartes coordinates system
pub struct CartesianGroup {
    data: Vec<(Series, Series)>,
    view: CView,
}

impl CartesianGroup {
    pub fn new() -> Self {
        Self {
            data: vec![],
            view: CView::default(),
        }
    }

    pub fn add_data(self, ax: Series, ay: Series) -> Self {
        let tuple = (ax, ay);
        let mut data = self.data;
        data.push(tuple);
        Self {
            data: data,
            view: self.view.clone(),
        }
    }

    pub fn set_view(
        &self,
        width: u64,
        height: u64,
        position_axes: usize,
        height_x_axis: u64,
        width_y_axis: u64,
        margin: u64,
    ) -> Self {
        let view = CView::new(
            width,
            height,
            position_axes,
            height_x_axis,
            width_y_axis,
            margin,
        );
        Self {
            data: self.data.clone(),
            view: view,
        }
    }

    pub fn get_view(&self) -> CView {
        self.view.clone()
    }

    pub fn get_ax_group(&self) -> Series {
        let first = &self.data[0].0;
        match first {
            Series::Number(s) => {
                let mut fix = s.clone();
                for index in 1..self.data.len() {
                    match &self.data[index].0 {
                        Series::Number(o) => fix = fix.merge(o.clone()),
                        Series::Label(_) => (),
                        Series::Time(_) => (),
                    }
                }
                return Series::Number(fix);
            }
            Series::Label(l) => {
                let mut fix = l.clone();
                for index in 1..self.data.len() {
                    match &self.data[index].0 {
                        Series::Number(_) => (),
                        Series::Label(l) => fix = fix.merge(l.clone()),
                        Series::Time(_) => (),
                    }
                }
                return Series::Label(fix);
            }
            Series::Time(s) => {
                let mut fix = s.clone();
                for index in 1..self.data.len() {
                    match &self.data[index].0 {
                        Series::Number(_) => (),
                        Series::Label(_) => (),
                        Series::Time(o) => fix = fix.merge(o.clone()),
                    }
                }
                return Series::Time(fix);
            }
        }
    }

    pub fn get_ay_group(&self) -> Series {
        let first = &self.data[0].1;
        match first {
            Series::Number(s) => {
                let mut fix = s.clone();
                for index in 1..self.data.len() {
                    match &self.data[index].1 {
                        Series::Number(o) => fix = fix.merge(o.clone()),
                        Series::Label(_) => (),
                        Series::Time(_) => (),
                    }
                }
                return Series::Number(fix);
            }
            Series::Label(l) => {
                let mut fix = l.clone();
                for index in 1..self.data.len() {
                    match &self.data[index].1 {
                        Series::Number(_) => (),
                        Series::Label(l) => fix = fix.merge(l.clone()),
                        Series::Time(_) => (),
                    }
                }
                return Series::Label(fix);
            }
            Series::Time(s) => {
                let mut fix = s.clone();
                for index in 1..self.data.len() {
                    match &self.data[index].1 {
                        Series::Number(_) => (),
                        Series::Label(_) => (),
                        Series::Time(o) => fix = fix.merge(o.clone()),
                    }
                }
                return Series::Time(fix);
            }
        }
    }

    pub fn get_data(&self) -> Vec<(Series, Series)> {
        self.data.clone()
    }
}
