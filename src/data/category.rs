#[derive(Debug, Clone)]
pub struct SCategory {
    series: Vec<String>
}

impl SCategory {
    pub fn new(series: Vec<String>) -> Self {
        Self{series}
    }

    pub fn get(&self) -> Vec<String> {
        self.series.clone()
    }
}