#![allow(unused)]

mod analysis;
mod graph;
mod landmarks;
mod parser;
mod ui;

use crate::parser::directed_or_undirected::DirectedOrUndirected;
use rand::Rng;
use rand::seq::SliceRandom;
use std::sync::atomic::Ordering;
use std::time;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    f32::consts::E,
    iter, vec,
};

#[tokio::main]
async fn main() {
    let file_name = ui::main_ui::run_ui_and_file_parcing_menu();
    let mut graph: graph::Graph =
        graph::Graph::new(parser::directed_or_undirected::DirectedOrUndirected::Undirected);
    match file_name {
        Some(path) => {
            let start_point = time::Instant::now();
            let (stop_animation, animation_handle) =
                ui::main_ui::spawn_cat_loading_animation(0, 0, Some(start_point));

            let parse_result = parser::parse::parse_file(&path).await;

            graph = match parse_result {
                Ok(graph) => graph,
                Err(error) => {
                    stop_animation.store(true, Ordering::Relaxed);
                    let _ = animation_handle.join();
                    panic!("Failed to parse the file: {error}");
                }
            };

            let degree_data: Vec<(f32, f32)> = analysis::degree::degree_probability(&graph);
            let log_degree_data: Vec<(f32, f32)> = analysis::degree::transform_to_log(&degree_data);

            stop_animation.store(true, Ordering::Relaxed);
            let _ = animation_handle.join();

            ui::degree_graphic_printing::print_graph(degree_data.clone());
            ui::degree_graphic_saving_in_png::save_graph_plotters(
                degree_data,
                Some("degree_data1"),
            )
            .expect("Failed to save graph as PNG");
            ui::degree_graphic_printing::print_graph(log_degree_data.clone());
            ui::degree_graphic_saving_in_png::save_graph_plotters(
                log_degree_data,
                Some("log_degree_data"),
            )
            .expect("Failed to save graph as PNG");
            println!("Time: {:.2?}", start_point.elapsed());
        }
        None => println!("No file selected. Exiting."),
    }
}
