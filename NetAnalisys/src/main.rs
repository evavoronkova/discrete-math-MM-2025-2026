#![allow(unused)]

mod analysis;
mod graph;
mod landmarks;
mod parser;
mod ui;

use crate::analysis::connectivity::{find_weak_components, fraction_in_largest_component, get_largest_comp, get_number_of_comps, tarjan_scc};
use crate::analysis::diameter::approximate_diameter;
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
            let mut buffer_for_print: Vec<String> = Vec::new();
            let graph_type = graph.kind();
            let num_vertices = graph.num_vertices();
            buffer_for_print.push(format!("Number of vertices in graph: {}", num_vertices));
            let num_edges = graph.num_edges();
            buffer_for_print.push(format!("Number of edges in graph: {}", num_edges));
            let density = graph.density(num_vertices, num_edges);
            buffer_for_print.push(format!("Density of graph: {}", density));
            let weak_comps = find_weak_components(&graph);
            let num_weak_comps = get_number_of_comps(&weak_comps);
            buffer_for_print.push(format!("Number of weak components: {}", num_weak_comps));
            let largest_weak_comp = get_largest_comp(&weak_comps);
            let fraction_in_weak_comp = fraction_in_largest_component(&largest_weak_comp, num_vertices);
            buffer_for_print.push(format!("Fraction in largest weak component: {}", fraction_in_weak_comp));
            if DirectedOrUndirected::Directed == graph_type {
                let strong_comps = tarjan_scc(&graph);
                let largest_strong_comp = get_largest_comp(&strong_comps);
                let num_strong_comps = get_number_of_comps(&strong_comps);
                buffer_for_print.push(format!("Number of strong components: {}", num_strong_comps));
                let fraction_in_strong_comp = fraction_in_largest_component(&largest_strong_comp, num_vertices);
                buffer_for_print.push(format!("Fraction in largest strong component: {}", fraction_in_strong_comp));
            }
            let diameter = approximate_diameter(&graph, Some(&largest_weak_comp));
            buffer_for_print.push(format!("Dieameter of largest weak component: {}", diameter));

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
