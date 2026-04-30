#[allow(unused)]
mod analysis;
mod graph;
mod landmarks;
mod parser;
mod ui;

use crate::analysis::cluster_evaluation::{
    calculate_global_k, calculate_mid_k, calculate_mid_k_for_weak_component,
};
use crate::analysis::connectivity::{
    find_weak_components, fraction_in_largest_component, get_largest_comp, get_number_of_comps,
    tarjan_scc,
};
use crate::analysis::degree::{all_degrees, max_degree, mid_degree, min_degree};
use crate::analysis::diameter::{self, approximate_diameter, percentile_90_distance};
use crate::analysis::triangle_counter::{self, find_triangles};
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

            buffer_for_print.push(("Type of the graph".to_string(), graph_type.to_string()));

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

            let (weak_comps, degree_data, strong_comps) = {
                let g1 = Arc::clone(&graph);
                let g2 = Arc::clone(&graph);
                let g3 = Arc::clone(&graph);
                tokio::try_join!(
                    task::spawn_blocking(move || find_weak_components(g1.as_ref())),
                    task::spawn_blocking(move || analysis::degree::degree_probability(g2.as_ref())),
                    async move {
                        Ok(if DirectedOrUndirected::Directed == graph_type {
                            Some(task::spawn_blocking(move || tarjan_scc(g3.as_ref())).await?)
                        } else {
                            None
                        })
                    }
                )
                .unwrap()
            };

            let weak_comps = Arc::new(weak_comps);

            let num_handle = {
                let weak_comps = Arc::clone(&weak_comps);
                task::spawn_blocking(move || get_number_of_comps(weak_comps.as_ref()))
            };

            let largest_handle = {
                let weak_comps = Arc::clone(&weak_comps);
                task::spawn_blocking(move || get_largest_comp(weak_comps.as_ref()))
            };

            let num_weak_comps = num_handle.await.unwrap();

            buffer_for_print.push((
                "Number of weak components".to_string(),
                num_weak_comps.to_string(),
            ));

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
                let strong_comps = strong_comps.unwrap();
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
            let largest_weak_comp = Arc::new(largest_weak_comp);

            let diameter_handle = {
                let graph = Arc::clone(&graph);
                let largest_weak_comp = Arc::clone(&largest_weak_comp);
                task::spawn_blocking(move || {
                    approximate_diameter(graph.as_ref(), Some(largest_weak_comp.as_ref()))
                })
            };

            let percentile_handle = {
                let graph = Arc::clone(&graph);
                task::spawn_blocking(move || percentile_90_distance(graph.as_ref(), None, 500))
            };

            let num_triangles_handle = {
                let graph = Arc::clone(&graph);
                task::spawn_blocking(move || find_triangles(graph.as_ref()))
            };

            let (diameter, percentile, num_triangles) =
                tokio::try_join!(diameter_handle, percentile_handle, num_triangles_handle).unwrap();

            buffer_for_print.push((
                "Diameter of largest weak component".to_string(),
                diameter.to_string(),
            ));

            buffer_for_print.push((
                "90 percentile of distance of the graph".to_string(),
                percentile.to_string(),
            ));

            buffer_for_print.push((
                "Number of triangles in graph".to_string(),
                num_triangles.to_string(),
            ));

            let mid_k_graph_handle = {
                let graph = Arc::clone(&graph);
                task::spawn_blocking(move || calculate_mid_k(graph.as_ref(), num_vertices))
            };

            let global_k_handle = {
                let graph = Arc::clone(&graph);
                task::spawn_blocking(move || calculate_global_k(graph.as_ref(), num_triangles))
            };

            let all_degrees_handle = {
                let graph = Arc::clone(&graph);
                task::spawn_blocking(move || all_degrees(graph.as_ref()))
            };

            let mid_k_component_handle = {
                let graph = Arc::clone(&graph);
                let largest_weak_comp = Arc::clone(&largest_weak_comp);
                task::spawn_blocking(move || {
                    calculate_mid_k_for_weak_component(graph.as_ref(), largest_weak_comp.as_ref())
                })
            };

            let (mid_k_graph, global_k, all_degrees, mid_k_component) = tokio::try_join!(
                mid_k_graph_handle,
                global_k_handle,
                all_degrees_handle,
                mid_k_component_handle
            )
            .unwrap();

            buffer_for_print.push((
                "Average cluster coefficient of the graph".to_string(),
                format!("{mid_k_graph:.6}"),
            ));

            buffer_for_print.push((
                "Global cluster coefficient of the graph".to_string(),
                format!("{global_k:.6}"),
            ));

            buffer_for_print.push((
                "Average cluster coefficient of the largest component".to_string(),
                format!("{mid_k_component:.6}"),
            ));

            let all_degrees = Arc::new(all_degrees);

            let max_degree_handle = {
                let degrees = Arc::clone(&all_degrees);
                task::spawn_blocking(move || max_degree(degrees.as_ref()))
            };

            let min_degree_handle = {
                let degrees = Arc::clone(&all_degrees);
                task::spawn_blocking(move || min_degree(degrees.as_ref()))
            };

            let mid_degree_handle = {
                let degrees = Arc::clone(&all_degrees);
                task::spawn_blocking(move || mid_degree(degrees.as_ref(), num_vertices))
            };

            let (max_degree, min_degree, mid_degree) =
                tokio::try_join!(max_degree_handle, min_degree_handle, mid_degree_handle).unwrap();

            buffer_for_print.push((
                "Maximal degree of the graph".to_string(),
                max_degree.to_string(),
            ));

            buffer_for_print.push((
                "Minimal degree of the graph".to_string(),
                min_degree.to_string(),
            ));

            buffer_for_print.push((
                "Average degree of the graph".to_string(),
                format!("{mid_degree:.6}"),
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
