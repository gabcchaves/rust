use std::convert::{From, Into, AsRef};

fn comp_median<T>(floats: Vec<T>) -> f32
where T: Into<f32> + std::ops::Add<Output=T> + Copy + std::cmp::Ord
{
    let ordered_floats = floats.sort();
    let median: f32;

    if floats.len() % 2 == 0 {
        let middle_indexes = [floats.len() / 2 - 1, floats.len() / 2];
        median = (middle_indexes[0] as f32 + middle_indexes[1] as f32) / 2 as f32;
    } else {
        median = (floats.len() / 2.0).floor();
    }

    (median * 10.0).round() / 10.0
}

fn main() {
    assert_eq!(comp_median(vec![1, 2, 3, 4, 5, 6, 7, 8]), 4.5);
}
