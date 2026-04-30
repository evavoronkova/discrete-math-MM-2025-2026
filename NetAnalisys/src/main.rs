#[allow(unused)]
mod analysis;
mod graph;
mod landmarks;
mod parser;
mod ui;

use crate::analysis::connectivity::{
    find_weak_components, fraction_in_largest_component, get_largest_comp, get_number_of_comps,
    tarjan_scc,
};
use crate::analysis::diameter::approximate_diameter;
use crate::parser::directed_or_undirected::DirectedOrUndirected;
use rayon::prelude::*;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::time;
use tokio::task;

fn print_table(data: &Vec<(String, String)>) {
    let metric_header = "Metric";
    let value_header = "Value";

    let metric_width = data
        .iter()
        .map(|(metric, _)| metric.chars().count())
        .max()
        .unwrap_or(0)
        .max(metric_header.len());
    let value_width = data
        .iter()
        .map(|(_, value)| value.chars().count())
        .max()
        .unwrap_or(0)
        .max(value_header.len());

    let top = format!(
        "╔{}╦{}╗",
        "═".repeat(metric_width + 2),
        "═".repeat(value_width + 2)
    );
    let separator = format!(
        "╠{}╬{}╣",
        "═".repeat(metric_width + 2),
        "═".repeat(value_width + 2)
    );
    let bottom = format!(
        "╚{}╩{}╝",
        "═".repeat(metric_width + 2),
        "═".repeat(value_width + 2)
    );

    println!("{top}");
    println!(
        "║ {:<metric_width$} ║ {:<value_width$} ║",
        metric_header, value_header
    );
    println!("{separator}");

    for (metric, value) in data {
        println!("║ {:<metric_width$} ║ {:<value_width$} ║", metric, value);
    }

    println!("{bottom}");
}

#[tokio::main]
async fn main() {
    let file_name = ui::main_ui::run_ui_and_file_parcing_menu();
    let mut graph: Arc<graph::Graph> = Arc::new(graph::Graph::new(
        parser::directed_or_undirected::DirectedOrUndirected::Undirected,
    ));
    match file_name {
        Some(path) => {
            let start_point = time::Instant::now();
            let (stop_animation, animation_handle) =
                ui::main_ui::spawn_cat_loading_animation(0, 0, Some(start_point));

            let parse_result = parser::parse::parse_file(&path).await;

            graph = match parse_result {
                Ok(graph) => Arc::new(graph),
                Err(error) => {
                    stop_animation.store(true, Ordering::Relaxed);
                    let _ = animation_handle.join();
                    panic!("Failed to parse the file: {error}");
                }
            };

            println!("[DEBUG] Graph successfully parsed. Starting analysis...");
            println!(
                "[DEBUG] Graph has {} vertices and {} edges.",
                graph.num_vertices(),
                graph.num_edges()
            );
            let mut buffer_for_print: Vec<(String, String)> = Vec::new();
            let graph_type = graph.kind();
            let num_vertices = graph.num_vertices();

            buffer_for_print.push((
                "Number of vertices in graph".to_string(),
                num_vertices.to_string(),
            ));

            let num_edges = graph.num_edges();

            buffer_for_print.push((
                "Number of edges in graph".to_string(),
                num_edges.to_string(),
            ));

            let density = graph.density(num_vertices, num_edges);

            buffer_for_print.push(("Density of graph".to_string(), format!("{density:.6}")));

            let weak_comps_handle = {
                let graph = Arc::clone(&graph);
                task::spawn_blocking(move || find_weak_components(&graph))
            };

            let degree_data_handle = {
                let graph = Arc::clone(&graph);
                task::spawn_blocking(move || analysis::degree::degree_probability(&graph))
            };
            let strong_comps_handle = if DirectedOrUndirected::Directed == graph_type {
                let graph = Arc::clone(&graph);
                Some(task::spawn_blocking(move || tarjan_scc(&graph)))
            } else {
                None
            };

            let weak_comps = Arc::new(weak_comps_handle.await.unwrap());
            let degree_data: Vec<(f32, f32)> = degree_data_handle.await.unwrap();

            let num_handle = {
                let weak_comps = Arc::clone(&weak_comps);
                task::spawn_blocking(move || get_number_of_comps(&weak_comps))
            };

            let largest_handle = {
                let weak_comps = Arc::clone(&weak_comps);
                task::spawn_blocking(move || get_largest_comp(&weak_comps))
            };

            let num_weak_comps = num_handle.await.unwrap();
            let largest_weak_comp = largest_handle.await.unwrap();

            let log_degree_data: Vec<(f32, f32)> = analysis::degree::transform_to_log(&degree_data);

            println!("[DEBUG] Largest weak components size: {}", num_weak_comps);
            println!(
                "[DEBUG] Largest weak component size: {}",
                largest_weak_comp.len()
            );
            buffer_for_print.push((
                "Fraction in largest weak component".to_string(),
                format!(
                    "{:.6}",
                    fraction_in_largest_component(&largest_weak_comp, num_vertices)
                ),
            ));
            if DirectedOrUndirected::Directed == graph_type {
                let strong_comps = strong_comps_handle.unwrap().await.unwrap();
                let largest_strong_comp = strong_comps
                    .par_iter()
                    .max_by_key(|comp| comp.len())
                    .unwrap()
                    .clone();
                let num_strong_comps = get_number_of_comps(&strong_comps);
                buffer_for_print.push((
                    "Number of strong components".to_string(),
                    num_strong_comps.to_string(),
                ));
                buffer_for_print.push((
                    "Fraction in largest strong component".to_string(),
                    format!(
                        "{:.6}",
                        fraction_in_largest_component(&largest_strong_comp, num_vertices)
                    ),
                ));
            }
            let diameter = {
                let graph = Arc::clone(&graph);
                task::spawn_blocking(move || approximate_diameter(&graph, Some(&largest_weak_comp)))
                    .await
                    .unwrap()
            };
            buffer_for_print.push((
                "Diameter of largest weak component".to_string(),
                diameter.to_string(),
            ));

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
            println!("\nGraph Analysis Results");
            print_table(&buffer_for_print);
            println!("Time: {:.2?}", start_point.elapsed());
        }
        None => println!("No file selected. Exiting."),
    }
}
