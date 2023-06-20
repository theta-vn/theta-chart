use crate::{
    chart::*,
    color::Color,
    coord::{Axes, Stick},
};

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

    pub fn labels(&self) -> Vec<String> {
        self.labels.clone()
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
    fn colors(&self) -> Vec<Color> {
        self.colors.clone()
    }

    // fn get_intervale(&self, len: f64) -> f64 {
    //     let distance = self.labels.len();
    //     len / (distance as f64)
    // }

    fn scale(&self, value: f64) -> f64 {
        let (min, max) = (0., self.labels.len() as f64);
        let range = max - min;

        let diff = value - min;
        diff / range
    }

    fn gen_axes(&self) -> Axes {
        let distance = self.labels.len();
        let series = &self.labels;
        let mut vec_stick: Vec<Stick> = vec![];
        // For stick < 0
        for index in 0..(distance) {
            let stick = Stick::new(format!("{}", series[index]), self.scale(index as f64 + 0.5));
            vec_stick.push(stick);
        }

        Axes {
            sticks: vec_stick,
            step: 1.,
        }
    }
    fn to_stick(&self) -> Vec<Stick> {
        let mut vec_stick: Vec<Stick> = vec![];
        let len = self.labels().len();
        for index in 0..len {
            let stick = Stick::new(format!("{}", self.labels()[index]), index as f64);
            vec_stick.push(stick);
        }
        vec_stick
    }
}
