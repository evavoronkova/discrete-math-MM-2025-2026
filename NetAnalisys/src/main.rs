#![allow(unused)]

mod analysis;
mod graph;
mod parser;
mod ui;
mod landmarks;

use crate::parser::directed_or_undirected::DirectedOrUndirected;
use rand::Rng;
use rand::seq::SliceRandom;
use std::{
    collections::{HashMap, HashSet, VecDeque}, iter, vec
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
#[tokio::main]
async fn main() {
    let file_name = ui::main_ui::run_ui_and_file_parcing_menu();
    let mut graph: graph::Graph =
        graph::Graph::new(parser::directed_or_undirected::DirectedOrUndirected::Undirected);
    match file_name {
        Some(path) => {
            graph = parser::parse::parse_file(&path).await.expect("Failed to parse the file");
            let degree_data: Vec<(f32, f32)> = analysis::degree::degree_probability(&graph);
            let log_degree_data: Vec<(f32, f32)> = analysis::degree::transform_to_log(&degree_data);

            ui::degree_graphic_printing::print_graph(degree_data.clone());
            ui::degree_graphic_saving_in_png::save_graph_plotters(degree_data, Some("degree_data1"))
                .expect("Failed to save graph as PNG");
            ui::degree_graphic_printing::print_graph(log_degree_data.clone());
            ui::degree_graphic_saving_in_png::save_graph_plotters(log_degree_data, Some("log_degree_data"))
                .expect("Failed to save graph as PNG");
        }
        None => println!("No file selected. Exiting."),
    }
}
