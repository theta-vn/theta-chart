use super::CView;
use crate::series::Series;

#[derive(Debug, Clone)]
/// Store data for descartes coordinates system
pub struct Cartesian {
    ax: Series,
    ay: Series,
    view: CView,
    error: String,
}

impl Cartesian {
    pub fn new(ax: Series, ay: Series) -> Self {
        let len_ax = ax.get_count();
        let len_ay = ay.get_count();
        let mut err = String::default();

        if len_ax != len_ay {
            err = "The lengths of the series are not equal".to_string()
        }

        Self {
            ax,
            ay,
            view: CView::default(),
            error: err,
        }
    }

    pub fn set_ax(self, ax: Series) -> Self {
        let len_ax = ax.get_count();
        let len_ay = self.ay.get_count();
        let mut err = String::default();
        if len_ax != len_ay {
            err = "The lengths of the series are not equal".to_string();
        }
        Self {
            ax: ax,
            ay: self.ay.clone(),
            view: self.view.clone(),
            error: err,
        }
    }

    pub fn set_ay(&self, ay: Series) -> Self {
        let len_ax = self.ax.get_count();
        let len_ay = ay.get_count();
        let mut err = String::default();
        if len_ax != len_ay {
            err = "The lengths of the series are not equal".to_string();
        }

        Self {
            ax: self.ax.clone(),
            ay: ay,
            view: self.view.clone(),
            error: err,
        }
    }

    pub fn get_error(&self) -> String {
        self.error.clone()
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
            ax: self.ax.clone(),
            ay: self.ay.clone(),
            view: view,
            error: self.error.clone(),
        }
    }

    pub fn get_view(&self) -> CView {
        self.view.clone()
    }

    pub fn get_ax(&self) -> Series {
        self.ax.clone()
    }

    pub fn get_ay(&self) -> Series {
        self.ay.clone()
    }
}
