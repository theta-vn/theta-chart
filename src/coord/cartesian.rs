use super::CView;
use crate::series::Series;

#[derive(Debug, Clone)]
/// Store data for descartes coordinates system
pub struct Cartesian {
    ax: Series,
    ay: Series,
    view: CView,
}

impl Cartesian {
    pub fn new(ax: Series, ay: Series) -> Self {
        Self {
            ax,
            ay,
            view: CView::default(),
        }
    }

    pub fn set_ax(self, ax: Series) -> Self {
        Self {
            ax: ax,
            ay: self.ay.clone(),
            view: self.view.clone(),
        }
    }

    pub fn set_ay(&self, ay: Series) -> Self {
        Self {
            ax: self.ax.clone(),
            ay: ay,
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
            ax: self.ax.clone(),
            ay: self.ay.clone(),
            view: view,
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
