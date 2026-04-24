#![allow(unused)]

mod analysis;
mod graph;
mod parser;
mod ui;
use crate::parser::directed_or_undirected::DirectedOrUndirected;
use rand::Rng;
use rand::seq::SliceRandom;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    vec,
};

fn test_data() -> HashMap<u32, f32> {
    let mut map = HashMap::new();

    map.insert(1, 100.0);
    map.insert(2, 50.0);
    map.insert(3, 25.0);
    map.insert(4, 12.5);
    map.insert(5, 6.25);
    map.insert(6, 3.12);
    map.insert(7, 1.56);
    map.insert(8, 0.78);

    map
}
fn main() {
    ui::main_ui::run_ui();
    ui::degree_graphic_printing::print_graph(
        test_data()
    );
    ui::degree_graphic_saving_in_png::save_graph_plotters(
        test_data()
    ).expect("Failed to save graph as PNG");
}
