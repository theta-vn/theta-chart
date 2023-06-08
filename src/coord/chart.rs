use crate::chart::{ScaleType};
use super::CView;

#[derive(Debug, Clone, Default)]
/// Store data for descartes coordinates system
pub struct Chart<X: ScaleType, Y: ScaleType> {
    ax: X,
    ay: Y,
    view: CView
}

impl<X: Clone + ScaleType, Y: Clone + ScaleType> Chart<X, Y> {
    pub fn set_ax(self, ax: X) -> Self {
        Self {
            ax: ax,
            ay: self.ay.clone(),
            view: self.view.clone()
        }
    }

    pub fn set_ay(&self, ay: Y) -> Self {
        Self {
            ax: self.ax.clone(),
            ay: ay,
            view: self.view.clone()
        }
    }

    pub fn set_view(&self, view: CView) -> Self {
        Self {
            ax: self.ax.clone(),
            ay: self.ay.clone(),
            view: view
        }
    }

    pub fn get_view(&self) -> CView {
        self.view.clone()
    }

    pub fn get_ax(&self) -> X {
        self.ax.clone()
    }

    pub fn get_ay(&self) -> Y {
        self.ay.clone()
    }

}
