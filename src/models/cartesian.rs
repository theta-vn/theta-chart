#[derive(Debug, Clone, Default)]
/// A Cartesian chart
pub struct Cartesian<T, U> {
    ax: T,
    ay: U,
}

impl<T: Clone, U: Clone> Cartesian<T, U> {
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
