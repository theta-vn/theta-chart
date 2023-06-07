use crate::chart::ScaleType;

#[derive(Debug, Clone, Default)]
/// Store data for descartes coordinates system
pub struct Chart<T: ScaleType, U: ScaleType> {
    ax: T,
    ay: U,
}

impl<T: Clone + ScaleType, U: Clone + ScaleType> Chart<T, U> {
    pub fn set_ax(self, ax: T) -> Self {
        Self {
            ax: ax,
            ay: self.ay.clone(),
        }
    }

    pub fn set_ay(&self, ay: U) -> Self {
        Self {
            ax: self.ax.clone(),
            ay: ay,
        }
    }
}
