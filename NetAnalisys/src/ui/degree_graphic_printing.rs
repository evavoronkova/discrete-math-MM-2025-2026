use std::collections::HashMap;

use textplots::{Chart, Plot, Shape};

use crate::analysis::degree;

pub fn transform_normal_to_log(degree_map: HashMap<u32, f32>) -> Vec<(f32, f32)> {
    degree_map
        .into_iter()
        .map(|(degree, count)| (f32::log10(degree as f32), f32::log10(count)))
        .collect()
}
pub fn print_graph(degree_map: HashMap<u32, f32>) {
    let mut data: Vec<(f32, f32)> = degree_map
        .into_iter()
        .map(|(degree, percent)| (degree as f32, percent))
        .collect();

    data.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let x_min = data.first().unwrap().0;
    let x_max = data.last().unwrap().0;
    println!("range X: [{:.2}, {:.2}]", x_min, x_max);
    Chart::new(180, 60, x_min, x_max)
        .lineplot(&Shape::Lines(&data))
        .display();
}
