use crate::{chart::*, color::Color};

// use super::ScaleLabel;

#[derive(Debug, Clone, Default)]
/// A series of labels represented on a chart
pub struct SLabel {
    labels: Vec<String>,
    colors: Vec<Color>,
}

fn gen_colors(num: usize) -> Vec<Color> {
    dbg!(&num);
    if num <= 2 {
        return vec![Color::default()];
    }
    let mut colors: Vec<Color> = vec![Color::default()];
    for index in 0..(num - 1) {
        let begin_color = colors[index].clone();

        let color = begin_color.shift_hue();
        dbg!(index, &begin_color, &color);
        colors.push(color.clone());
    }
    colors
}

impl SLabel {
    pub fn new(labels: Vec<String>, colors: Vec<Color>) -> Self {
        Self { labels, colors }
    }
}

impl From<Vec<String>> for SLabel {
    fn from(labes: Vec<String>) -> Self {
        let len = labes.len();
        let colors = gen_colors(len);
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
        // let a = gen_colors(len);
        let colors = gen_colors(len);
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
