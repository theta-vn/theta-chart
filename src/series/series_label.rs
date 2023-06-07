use crate::{chart::*, color::Color};

// use super::ScaleLabel;

#[derive(Debug, Clone, Default)]
/// A series of labels represented on a chart
pub struct SLabel {
    labels: Vec<String>,
    colors: Vec<Color>,
}

impl SLabel {
    pub fn new(labels: Vec<String>, colors: Vec<Color>) -> Self {
        Self { labels, colors }
    }
}

impl From<Vec<String>> for SLabel {
    fn from(labes: Vec<String>) -> Self {
        let len = labes.len();
        let colors = vec![Color::default(); len];
        Self {
            labels: labes.clone(),
            colors: colors,
        }
    }
}

impl From<Vec<&str>> for SLabel {
    fn from(labes: Vec<&str>) -> Self {
        let vec_string: Vec<String> = labes.iter().map(|&s| s.into()).collect();
        let len = labes.len();
        let colors = vec![Color::default(); len];
        Self {
            labels: vec_string,
            colors: colors,
        }
    }
}

impl ScaleLabel for SLabel {
    fn labels(&self) -> Vec<String> {
        self.labels.clone()
    }

    fn colors(&self) -> Vec<Color> {
        self.colors.clone()
    }
}

impl ScaleType for SLabel {
    fn scale_type(&self) -> String {
        "ScaleLabel".to_string()
    }
}
