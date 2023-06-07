use crate::chart::{ScaleType};
use super::View;

#[derive(Debug, Clone, Default)]
/// Store data for descartes coordinates system
pub struct Chart<X: ScaleType, Y: ScaleType> {
    ax: X,
    ay: Y,
    view: View
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

    pub fn set_view(&self, view: View) -> Self {
        Self {
            ax: self.ax.clone(),
            ay: self.ay.clone(),
            view: view
        }
    }
}
