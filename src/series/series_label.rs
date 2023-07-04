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
    // dbg!(&num);
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

    pub fn merge(&self, other: SLabel) -> Self {
        let v1 = self.labels();
        let v2 = other.labels();
        let labels = merge_vec_string(v1, v2);

        let len = labels.len();
        let colors = gen_colors(len);
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
    fn colors(&self) -> Vec<Color> {
        self.colors.clone()
    }

    fn scale(&self, value: f64) -> f64 {
        let (min, max) = (0., self.labels.len() as f64);
        let range = max - min;

        let diff = value - min;
        diff / range
    }

    fn scale_index(&self, s: String) -> usize {
        let labels = self.labels();
        get_index(&labels, s)
        // 1.
    }

    fn gen_axes(&self) -> Axes {
        let style = "label".to_string();
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
            style,
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

fn merge_vec_string(v1: Vec<String>, v2: Vec<String>) -> Vec<String> {
    let dup = get_dup(v1.clone(), v2.clone());
    let mut result: Vec<String> = vec![];

    if dup.len() == 0 {
        result.extend(v1);
        result.extend(v2);
    } else {
        let s = dup[0].clone();
        let dup_index_v1 = get_index(&v1, s.clone());
        let dup_index_v2 = get_index(&v2, s.clone());
        let mut left_v1 = v1.clone();
        let mut left_v2 = v2.clone();
        let mut right_v1 = v1.clone();
        let mut right_v2 = v2.clone();

        left_v1.truncate(dup_index_v1);
        left_v2.truncate(dup_index_v2);
        let mut left = merge_vec_string(left_v1, left_v2);
        left.push(s);

        right_v1 = right_v1.split_off(dup_index_v1 + 1);
        right_v2 = right_v2.split_off(dup_index_v2 + 1);

        let right = merge_vec_string(right_v1, right_v2);

        result = merge_vec_string(left, right)
    }
    result
}

fn get_dup(v1: Vec<String>, v2: Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    for inv2 in v2 {
        if v1.contains(&inv2) {
            result.push(inv2)
        }
    }
    result
}

fn get_index(vec: &Vec<String>, s: String) -> usize {
    let index = vec.iter().position(|r| r == &s).unwrap();
    index
}
