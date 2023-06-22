use super::PView;
use crate::series::{SLabel, SNumber, Series};

#[derive(Debug, Clone)]
/// Store data for polar coordinates system
pub struct Polar {
    data: Series,
    label: Series,
    view: PView,
}

impl Polar {
    pub fn new(data: Series, label: Series) -> Self {
        Self {
            data,
            label,
            view: PView::default(),
        }
    }

    pub fn set_data(self, data: Series) -> Self {
        Self {
            data: data,
            label: self.label.clone(),
            view: self.view.clone(),
        }
    }

    pub fn set_label(&self, label: Series) -> Self {
        Self {
            data: self.data.clone(),
            label: label,
            view: self.view.clone(),
        }
    }

    pub fn set_view(
        &self,
        width: u64,
        height: u64,
        position_label: usize,
        width_label: u64,
        margin: u64,
    ) -> Self {
        let view = PView::new(width, height, position_label, width_label, margin);
        Self {
            data: self.data.clone(),
            label: self.label.clone(),
            view: view,
        }
    }

    pub fn get_view(&self) -> PView {
        self.view.clone()
    }

    pub fn get_data(&self) -> SNumber {
        match &self.data {
            Series::Number(n) => n.clone(),
            Series::Label(_) => SNumber::default(),
            Series::Time(_) => SNumber::default(),
        }
    }

    pub fn get_label(&self) -> SLabel {
        match &self.label {
            Series::Number(_) => SLabel::default(),
            Series::Label(l) => l.clone(),
            Series::Time(_) => SLabel::default(),
        }
    }
}
