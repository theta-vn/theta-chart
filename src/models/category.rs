use crate::ScaleCategory;

#[derive(Debug, Clone)]
pub struct Category {
    labels: Vec<String>,
}

impl Category {
    pub fn new(series: Vec<String>) -> Self {
        Self{labels: series}
    }
}

impl ScaleCategory for Category {
    fn labels(&self) -> Vec<String> {
        self.labels.clone()
    }
}