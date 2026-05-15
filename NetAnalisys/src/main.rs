#[allow(unused)]
mod analysis;
mod graph;
mod interactive_landmarks;
mod landmarks;
mod parser;
mod ui;

use crate::analysis::cluster_evaluation::{
    calculate_global_k_from_stats, calculate_mid_k_from_stats,
    calculate_mid_k_from_stats_for_component,
};
use crate::analysis::connectivity::{
    find_weak_components, fraction_from_component_size, fraction_in_largest_component,
    get_largest_comp, get_number_of_comps, tarjan_scc,
};
use crate::analysis::degree::{all_degrees, max_degree, mid_degree, min_degree};
use crate::analysis::diameter::{count_diameters, percentile_90_distance};
use crate::analysis::robustness::{lcc_after_hub_removal, lcc_after_random_removal};
use crate::analysis::triangle_counter::{compute_triangle_stats, find_triangles};
use crate::parser::directed_or_undirected::DirectedOrUndirected;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::Ordering;
use std::time::{Duration, Instant};
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

fn open_perf_log() -> Arc<Mutex<std::fs::File>> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("performance.log")
        .expect("Failed to open performance.log");

    Arc::new(Mutex::new(file))
}

fn log_duration(log: &Arc<Mutex<std::fs::File>>, label: &str, elapsed: Duration) {
    let mut file = log.lock().unwrap();
    writeln!(file, "{label}\t{elapsed:.6?}").unwrap();
}

// async fn time_async<T, F>(log: &Arc<Mutex<std::fs::File>>, label: &'static str, fut: F) -> T
// where
//     F: Future<Output = T>,
// {
//     let start = Instant::now();
//     let out = fut.await;
//     log_duration(log, label, start.elapsed());
//     out
// }

fn spawn_blocking_logged<T, F>(
    log: Arc<Mutex<std::fs::File>>,
    label: &'static str,
    job: F,
) -> tokio::task::JoinHandle<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    task::spawn_blocking(move || {
        let start = Instant::now();
        let result = job();
        log_duration(&log, label, start.elapsed());
        result
    })
}

#[tokio::main]
async fn main() {
    let perf_log = open_perf_log();
    let file_name = ui::main_ui::run_ui_and_file_parsing_menu();
    let graph: Arc<graph::Graph>;
    match file_name {
        Some(path) => {
            let start_point = Instant::now();
            let (stop_animation, animation_handle) =
                ui::main_ui::spawn_cat_loading_animation(0, 0, Some(start_point));

            let start_parse = Instant::now();
            let parse_result = parser::parse::parse_file(&path);
            log_duration(&perf_log, "parse_file", start_parse.elapsed());

            graph = match parse_result {
                Ok(graph) => Arc::new(graph),
                Err(error) => {
                    stop_animation.store(true, Ordering::Relaxed);
                    let _ = animation_handle.join();
                    eprintln!("Error: failed to parse file: {error}");
                    return;
                }
            };

            // println!("[DEBUG] Graph successfully parsed. Starting analysis...");
            // println!(
            //     "[DEBUG] Graph has {} vertices and {} edges.",
            //     graph.num_vertices(),
            //     graph.num_edges()
            // );
            let mut buffer_for_print_default_info: Vec<(String, String)> = Vec::new();
            let graph_type = graph.kind();

            buffer_for_print_default_info
                .push(("Type of the graph".to_string(), graph_type.to_string()));

            let num_vertices = graph.num_vertices();

            buffer_for_print_default_info.push((
                "Number of vertices in graph".to_string(),
                num_vertices.to_string(),
            ));

            let num_edges = graph.num_edges();

            buffer_for_print_default_info.push((
                "Number of edges in graph".to_string(),
                num_edges.to_string(),
            ));

            let density = graph.density(num_vertices, num_edges);

            buffer_for_print_default_info
                .push(("Density of graph".to_string(), format!("{density:.6}")));

            let (weak_comps, degree_data, strong_comps) = {
                let g1 = Arc::clone(&graph);
                let g2 = Arc::clone(&graph);
                let g3 = Arc::clone(&graph);
                let perf_log_weak = Arc::clone(&perf_log);
                let perf_log_degree = Arc::clone(&perf_log);
                let perf_log_tarjan = Arc::clone(&perf_log);
                tokio::try_join!(
                    spawn_blocking_logged(perf_log_weak, "find_weak_components", move || {
                        find_weak_components(g1.as_ref())
                    }),
                    spawn_blocking_logged(perf_log_degree, "degree_probability", move || {
                        analysis::degree::degree_probability(g2.as_ref())
                    }),
                    async move {
                        Ok(if DirectedOrUndirected::Directed == graph_type {
                            Some(
                                spawn_blocking_logged(perf_log_tarjan, "tarjan_scc", move || {
                                    tarjan_scc(g3.as_ref())
                                })
                                .await?,
                            )
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
                spawn_blocking_logged(Arc::clone(&perf_log), "get_number_of_comps", move || {
                    get_number_of_comps(weak_comps.as_ref())
                })
            };

            let largest_handle = {
                let weak_comps = Arc::clone(&weak_comps);
                spawn_blocking_logged(Arc::clone(&perf_log), "get_largest_comp", move || {
                    get_largest_comp(weak_comps.as_ref())
                })
            };

            let num_weak_comps = num_handle.await.unwrap();

            buffer_for_print_default_info.push((
                "Number of weak components".to_string(),
                num_weak_comps.to_string(),
            ));

            let largest_weak_comp = largest_handle.await.unwrap();

            let log_degree_data: Vec<(f32, f32)> = analysis::degree::transform_to_log(&degree_data);

            // println!("[DEBUG] Largest weak components size: {}", num_weak_comps);
            // println!(
            //     "[DEBUG] Largest weak component size: {}",
            //     largest_weak_comp.len()
            // );
            buffer_for_print_default_info.push((
                "Fraction in largest weak component".to_string(),
                format!(
                    "{:.6}",
                    fraction_in_largest_component(&largest_weak_comp, num_vertices)
                ),
            ));
            if DirectedOrUndirected::Directed == graph_type {
                let strong_comps = strong_comps.unwrap();
                let largest_strong_comp_size = strong_comps
                    .iter()
                    .map(|comp| comp.len())
                    .max()
                    .unwrap_or(0);
                let num_strong_comps = get_number_of_comps(&strong_comps);
                buffer_for_print_default_info.push((
                    "Number of strong components".to_string(),
                    num_strong_comps.to_string(),
                ));
                buffer_for_print_default_info.push((
                    "Fraction in largest strong component".to_string(),
                    format!(
                        "{:.6}",
                        fraction_from_component_size(largest_strong_comp_size, num_vertices)
                    ),
                ));
            }
            let largest_weak_comp = Arc::new(largest_weak_comp);

            let percentile_handle = {
                let graph = Arc::clone(&graph);
                spawn_blocking_logged(Arc::clone(&perf_log), "percentile_90_distance", move || {
                    percentile_90_distance(graph.as_ref(), None, 500)
                })
            };

            let num_triangles_handle = {
                let graph = Arc::clone(&graph);
                spawn_blocking_logged(Arc::clone(&perf_log), "find_triangles", move || {
                    find_triangles(graph.as_ref())
                })
            };

            let (percentile, num_triangles) =
                tokio::try_join!(percentile_handle, num_triangles_handle).unwrap();

            let diameters =
                count_diameters(Arc::clone(&graph), Some(Arc::clone(&largest_weak_comp))).await;

            buffer_for_print_default_info.push((
                "Diameter of largest weak component on double bfs".to_string(),
                diameters[0].to_string(),
            ));

            buffer_for_print_default_info.push((
                "Diameter of largest weak component on random vertices".to_string(),
                diameters[1].to_string(),
            ));

            buffer_for_print_default_info.push((
                "Diameter of largest weak component on snowball sampling".to_string(),
                diameters[2].to_string(),
            ));

            buffer_for_print_default_info.push((
                "90 percentile of distance of the graph".to_string(),
                percentile.to_string(),
            ));

            buffer_for_print_default_info.push((
                "Number of triangles in graph".to_string(),
                num_triangles.to_string(),
            ));

            let triangle_stats_handle = {
                let graph = Arc::clone(&graph);
                spawn_blocking_logged(Arc::clone(&perf_log), "compute_triangle_stats", move || {
                    compute_triangle_stats(graph.as_ref(), None)
                })
            };

            let all_degrees_handle = {
                let graph = Arc::clone(&graph);
                spawn_blocking_logged(Arc::clone(&perf_log), "all_degrees", move || {
                    all_degrees(graph.as_ref())
                })
            };

            let (triangle_stats, all_degrees) =
                tokio::try_join!(triangle_stats_handle, all_degrees_handle).unwrap();

            let triangle_stats = Arc::new(triangle_stats);

            let mid_k_graph = {
                let triangle_stats = Arc::clone(&triangle_stats);
                spawn_blocking_logged(Arc::clone(&perf_log), "calculate_mid_k", move || {
                    calculate_mid_k_from_stats(&triangle_stats, num_vertices)
                })
            };

            let global_k = {
                let triangle_stats = Arc::clone(&triangle_stats);
                spawn_blocking_logged(Arc::clone(&perf_log), "calculate_global_k", move || {
                    calculate_global_k_from_stats(&triangle_stats, num_triangles)
                })
            };

            let mid_k_component = {
                let triangle_stats = Arc::clone(&triangle_stats);
                let largest_weak_comp = Arc::clone(&largest_weak_comp);
                let graph = Arc::clone(&graph);
                spawn_blocking_logged(
                    Arc::clone(&perf_log),
                    "calculate_mid_k_for_weak_component",
                    move || {
                        calculate_mid_k_from_stats_for_component(
                            &triangle_stats,
                            graph.as_ref(),
                            largest_weak_comp.as_ref(),
                        )
                    },
                )
            };

            let (mid_k_graph, global_k, mid_k_component) =
                tokio::try_join!(mid_k_graph, global_k, mid_k_component).unwrap();

            buffer_for_print_default_info.push((
                "Average cluster coefficient of the graph".to_string(),
                format!("{mid_k_graph:.6}"),
            ));

            buffer_for_print_default_info.push((
                "Global cluster coefficient of the graph".to_string(),
                format!("{global_k:.6}"),
            ));

            buffer_for_print_default_info.push((
                "Average cluster coefficient of the largest weak component".to_string(),
                format!("{mid_k_component:.6}"),
            ));

            let all_degrees = Arc::new(all_degrees);

            let max_degree_handle = {
                let degrees = Arc::clone(&all_degrees);
                spawn_blocking_logged(Arc::clone(&perf_log), "max_degree", move || {
                    max_degree(degrees.as_ref())
                })
            };

            let min_degree_handle = {
                let degrees = Arc::clone(&all_degrees);
                spawn_blocking_logged(Arc::clone(&perf_log), "min_degree", move || {
                    min_degree(degrees.as_ref())
                })
            };

            let mid_degree_handle = {
                let degrees = Arc::clone(&all_degrees);
                spawn_blocking_logged(Arc::clone(&perf_log), "mid_degree", move || {
                    mid_degree(degrees.as_ref(), num_vertices)
                })
            };

            let (max_degree, min_degree, mid_degree) =
                tokio::try_join!(max_degree_handle, min_degree_handle, mid_degree_handle).unwrap();

            buffer_for_print_default_info.push((
                "Maximal degree of the graph".to_string(),
                max_degree.to_string(),
            ));

            buffer_for_print_default_info.push((
                "Minimal degree of the graph".to_string(),
                min_degree.to_string(),
            ));

            buffer_for_print_default_info.push((
                "Average degree of the graph".to_string(),
                format!("{mid_degree:.6}"),
            ));

            let mut buffer_for_print_random_removes: Vec<(String, String)> = Vec::new();
            let mut buffer_for_print_removes_of_largest: Vec<(String, String)> = Vec::new();

            let lcc_random_removes_handle = {
                let graph = Arc::clone(&graph);
                spawn_blocking_logged(
                    Arc::clone(&perf_log),
                    "lcc_after_random_removal",
                    move || lcc_after_random_removal(graph.as_ref(), num_vertices, 10),
                )
            };

            let lcc_hub_removes_handle = {
                let graph = Arc::clone(&graph);
                let degrees = Arc::clone(&all_degrees);
                spawn_blocking_logged(Arc::clone(&perf_log), "lcc_after_hub_removal", move || {
                    lcc_after_hub_removal(graph.as_ref(), num_vertices, degrees.as_ref())
                })
            };

            let (lcc_random_removes, lcc_hub_removes) =
                tokio::try_join!(lcc_random_removes_handle, lcc_hub_removes_handle).unwrap();

            for i in 1..=20 as u32 {
                let percent = i * 5;
                let fraction_random = lcc_random_removes[&percent];
                let fraction_hub = lcc_hub_removes[&percent];
                buffer_for_print_random_removes
                    .push((format!("{percent}%"), fraction_random.to_string()));
                buffer_for_print_removes_of_largest
                    .push((format!("{percent}%"), fraction_hub.to_string()));
            }

            stop_animation.store(true, Ordering::Relaxed);
            let _ = animation_handle.join();

            {
                let start = Instant::now();
                ui::degree_graphic_printing::print_graph(&degree_data);
                log_duration(&perf_log, "print_degree_graph", start.elapsed());
            }
            {
                let start = Instant::now();
                ui::degree_graphic_saving_in_png::save_graph_plotters(
                    &degree_data,
                    Some("degree_data1"),
                )
                .expect("Failed to save graph as PNG");
                log_duration(&perf_log, "save_degree_graph_png", start.elapsed());
            }
            {
                let start = Instant::now();
                ui::degree_graphic_printing::print_graph(&log_degree_data);
                log_duration(&perf_log, "print_log_degree_graph", start.elapsed());
            }
            {
                let start = Instant::now();
                ui::degree_graphic_saving_in_png::save_graph_plotters(
                    &log_degree_data,
                    Some("log_degree_data"),
                )
                .expect("Failed to save graph as PNG");
                log_duration(&perf_log, "save_log_degree_graph_png", start.elapsed());
            }
            println!(
                "\nGraph analysis ended succesfully, do you want to see results or make requests for estimating distance? "
            );
            print!("y/n/yes/no> ");
            let mut ans = String::new();
            std::io::stdin().read_line(&mut ans);
            let ans = ans.as_str();

            println!("\nGraph Analysis Results");
            {
                let start = Instant::now();
                print_table(&buffer_for_print_default_info);
                println!("\nFraction of largest weak component after remove random n% vertices");
                print_table(&buffer_for_print_random_removes);
                println!(
                    "\nFraction of largest weak component after remove n% vertices with largest degree"
                );
                print_table(&buffer_for_print_removes_of_largest);
                log_duration(&perf_log, "print_result_table", start.elapsed());
            }
            log_duration(&perf_log, "total_runtime", start_point.elapsed());
            println!("Time: {:.2?}", start_point.elapsed());
        }
        None => println!("No file selected. Exiting."),
    }
}
