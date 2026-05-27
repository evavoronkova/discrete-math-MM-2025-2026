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
use crate::analysis::degree::{
    all_degrees, degree_probability_vec, degrees_internal, max_degree, mid_degree, min_degree,
};
use crate::analysis::diameter::{DiameterMethod, count_diameters, percentile_90_distance};
use crate::analysis::robustness::{lcc_after_hub_removal, lcc_after_random_removal};
use crate::analysis::triangle_counter::{compute_triangle_stats, find_triangles};
use crate::parser::directed_or_undirected::DirectedOrUndirected;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::task;

pub fn print_table(data: &Vec<(String, String)>) {
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

struct PerfLogger {
    file: Arc<Mutex<std::fs::File>>,
    file_name: String,
}

impl PerfLogger {
    fn new(file: Arc<Mutex<std::fs::File>>, file_name: String) -> Self {
        Self { file, file_name }
    }

    /// Create a clone for use in separate spawned tasks (shares the same file handle).
    fn clone_for_spawn(&self) -> Self {
        Self {
            file: Arc::clone(&self.file),
            file_name: self.file_name.clone(),
        }
    }

    fn log_duration(&self, label: &str, elapsed: Duration) {
        let mut f = self.file.lock().unwrap();
        let _ = writeln!(f, "{}\t{}\t{:.6?}", self.file_name, label, elapsed);
    }

    /// Spawn a blocking task and log its execution duration.
    /// The label must be `&'static str` so it can be moved into the spawned closure.
    fn spawn_blocking<T, F>(&self, label: &'static str, job: F) -> tokio::task::JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let file = Arc::clone(&self.file);
        let file_name = self.file_name.clone();
        task::spawn_blocking(move || {
            let start = Instant::now();
            let result = job();
            let mut f = file.lock().unwrap();
            let _ = writeln!(f, "{}\t{}\t{:.6?}", file_name, label, start.elapsed());
            result
        })
    }
}

fn open_perf_log() -> Arc<Mutex<std::fs::File>> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("performance.log")
        .expect("Failed to open performance.log");
    Arc::new(Mutex::new(file))
}

fn open_trace_log() -> Arc<Mutex<std::fs::File>> {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("trace.log")
        .expect("Failed to open trace.log");
    Arc::new(Mutex::new(file))
}

fn log_start(
    trace_log: &Arc<Mutex<std::fs::File>>,
    file_name: &str,
    start_point: Instant,
    label: &str,
) {
    let elapsed = start_point.elapsed();
    let mut file = trace_log.lock().unwrap();
    let _ = writeln!(file, "{}\tSTART\t{}\t{:.3?}", file_name, label, elapsed);
}

#[tokio::main]
async fn main() {
    let perf_log = open_perf_log();
    let trace_log = open_trace_log();
    let file_path = ui::main_ui::run_ui_and_file_parsing_menu();
    let graph: Arc<graph::Graph>;
    match file_path {
        Some(path) => {
            // Extract just the filename for logging purposes
            let file_name = std::path::Path::new(&path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(&path)
                .to_string();

            // Choose diameter method before starting computations
            let diameter_method =
                ui::main_ui::select_diameter_method().unwrap_or(DiameterMethod::All);

            let start_point = Instant::now();
            let (stop_animation, animation_handle) =
                ui::main_ui::spawn_cat_loading_animation(0, 0, Some(start_point));

            // Write header to performance log
            {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(0);
                let mut f = perf_log.lock().unwrap();
                let _ = writeln!(
                    f,
                    "# ===== Run: {} @ {} =====",
                    file_name,
                    format_timestamp(now)
                );
            }

            // ── Parse graph file ─────────────────────────────────────────
            log_start(
                &trace_log,
                &file_name,
                start_point,
                "Parse graph file from disk",
            );
            let start_parse = Instant::now();
            let parse_result = parser::parse::parse_file(&path);
            let parse_duration = start_parse.elapsed();
            // Log parse duration directly (we don't have a PerfLogger yet)
            {
                let mut f = perf_log.lock().unwrap();
                let _ = writeln!(
                    f,
                    "{}\tParse graph file from disk\t{:.6?}",
                    file_name, parse_duration
                );
            }

            graph = match parse_result {
                Ok(graph) => Arc::new(graph),
                Err(error) => {
                    stop_animation.store(true, Ordering::Relaxed);
                    let _ = animation_handle.join();
                    eprintln!("Error: failed to parse file: {error}");
                    return;
                }
            };

            // Create the structured logger now that we know the file parsed
            let logger = PerfLogger::new(perf_log, file_name.clone());

            // Write graph metadata header
            {
                let mut f = logger.file.lock().unwrap();
                let _ = writeln!(
                    f,
                    "# Vertices: {} | Edges: {} | Type: {} | Density: {:.6}",
                    graph.num_vertices(),
                    graph.num_edges(),
                    graph.kind(),
                    graph.density(graph.num_vertices(), graph.num_edges()),
                );
                let _ = writeln!(f, "#");
            }

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

            // ── Phase 1: Weak components, degree distribution, SCC ─────────
            let (weak_comps, degree_data, strong_comps) = {
                let g1 = Arc::clone(&graph);
                let g2 = Arc::clone(&graph);
                let g3 = Arc::clone(&graph);
                let logger_weak = logger.clone_for_spawn();
                let logger_degree = logger.clone_for_spawn();
                let logger_tarjan = logger.clone_for_spawn();
                log_start(
                    &trace_log,
                    &file_name,
                    start_point,
                    "Find weak connectivity components",
                );
                log_start(
                    &trace_log,
                    &file_name,
                    start_point,
                    "Compute degree probability distribution",
                );
                log_start(
                    &trace_log,
                    &file_name,
                    start_point,
                    "Find strongly connected components (Tarjan's SCC)",
                );
                tokio::try_join!(
                    logger_weak.spawn_blocking("Find weak connectivity components", move || {
                        find_weak_components(g1.as_ref())
                    }),
                    logger_degree
                        .spawn_blocking("Compute degree probability distribution", move || {
                            degree_probability_vec(g2.as_ref())
                        }),
                    async move {
                        Ok(if DirectedOrUndirected::Directed == graph_type {
                            Some(
                                logger_tarjan
                                    .spawn_blocking(
                                        "Find strongly connected components (Tarjan's SCC)",
                                        move || tarjan_scc(g3.as_ref()),
                                    )
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

            // ── Phase 2: Component statistics ─────────────────────────────
            let num_handle = {
                let weak_comps = Arc::clone(&weak_comps);
                logger.spawn_blocking("Count number of weak components", move || {
                    get_number_of_comps(weak_comps.as_ref())
                })
            };

            let largest_handle = {
                let weak_comps = Arc::clone(&weak_comps);
                logger.spawn_blocking("Extract largest weak component", move || {
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

            // ── Phase 3: Percentile distance + triangle counting ───────────
            log_start(
                &trace_log,
                &file_name,
                start_point,
                "Compute 90th percentile distance (random sampling)",
            );
            let percentile_handle = {
                let graph = Arc::clone(&graph);
                let component = Arc::clone(&largest_weak_comp);
                logger.spawn_blocking(
                    "Compute 90th percentile distance (random sampling)",
                    move || percentile_90_distance(graph.as_ref(), Some(component.as_ref()), 500),
                )
            };

            log_start(
                &trace_log,
                &file_name,
                start_point,
                "Count triangles in graph",
            );
            let num_triangles_handle = {
                let graph = Arc::clone(&graph);
                logger.spawn_blocking("Count triangles in graph", move || {
                    find_triangles(graph.as_ref())
                })
            };

            let (percentile, num_triangles) =
                tokio::try_join!(percentile_handle, num_triangles_handle).unwrap();

            // ── Phase 4: Diameter estimation ──────────────────────────────
            log_start(
                &trace_log,
                &file_name,
                start_point,
                "Estimate graph diameter",
            );
            let (diameter_double, diameter_random, diameter_snowball) = match diameter_method {
                DiameterMethod::DoubleBfs => {
                    let g = Arc::clone(&graph);
                    let c = Some(Arc::clone(&largest_weak_comp));
                    let d = logger
                        .spawn_blocking("Compute approximate diameter (double BFS)", move || {
                            analysis::diameter::approximate_diameter(g.as_ref(), c.as_deref())
                        })
                        .await
                        .unwrap();
                    (Some(d), None, None)
                }
                DiameterMethod::RandomLike => {
                    let g = Arc::clone(&graph);
                    let c = Some(Arc::clone(&largest_weak_comp));
                    let d = logger
                        .spawn_blocking(
                            "Compute diameter (random sampling, 500 iterations)",
                            move || {
                                analysis::diameter::random_like_diameter_calculate(
                                    g.as_ref(),
                                    c.as_deref(),
                                    500,
                                )
                            },
                        )
                        .await
                        .unwrap();
                    (None, Some(d), None)
                }
                DiameterMethod::Snowball => {
                    let g = Arc::clone(&graph);
                    let c = Some(Arc::clone(&largest_weak_comp));
                    let d = logger
                        .spawn_blocking(
                            "Compute diameter (snowball sampling, 1000 seeds)",
                            move || {
                                analysis::diameter::snowball_sampling(
                                    g.as_ref(),
                                    c.as_deref(),
                                    1000,
                                )
                            },
                        )
                        .await
                        .unwrap();
                    (None, None, Some(d))
                }
                DiameterMethod::All => {
                    let res = count_diameters(
                        Arc::clone(&graph),
                        Some(Arc::clone(&largest_weak_comp)),
                        file_name.clone(),
                        Arc::clone(&logger.file),
                    )
                    .await;
                    (Some(res[0]), Some(res[1]), Some(res[2]))
                }
            };

            if let Some(d) = diameter_double {
                buffer_for_print_default_info.push((
                    "Diameter of largest weak component on double bfs".to_string(),
                    d.to_string(),
                ));
            }
            if let Some(d) = diameter_random {
                buffer_for_print_default_info.push((
                    "Diameter of largest weak component on random vertices".to_string(),
                    d.to_string(),
                ));
            }
            if let Some(d) = diameter_snowball {
                buffer_for_print_default_info.push((
                    "Diameter of largest weak component on snowball sampling".to_string(),
                    d.to_string(),
                ));
            }

            buffer_for_print_default_info.push((
                "90 percentile of distance of the graph".to_string(),
                percentile.to_string(),
            ));

            buffer_for_print_default_info.push((
                "Number of triangles in graph".to_string(),
                num_triangles.to_string(),
            ));

            // ── Phase 5: Triangle stats + degrees ─────────────────────────
            let triangle_stats_handle = {
                let graph = Arc::clone(&graph);
                logger.spawn_blocking("Compute triangle statistics for clustering", move || {
                    compute_triangle_stats(graph.as_ref(), None)
                })
            };

            let all_degrees_handle = {
                let graph = Arc::clone(&graph);
                logger.spawn_blocking("Extract all vertex degrees", move || {
                    all_degrees(graph.as_ref())
                })
            };

            let (triangle_stats, all_degrees) =
                tokio::try_join!(triangle_stats_handle, all_degrees_handle).unwrap();

            let triangle_stats = Arc::new(triangle_stats);

            // ── Phase 6: Clustering coefficients ───────────────────────────
            log_start(
                &trace_log,
                &file_name,
                start_point,
                "Compute clustering coefficients",
            );
            let mid_k_graph = {
                let triangle_stats = Arc::clone(&triangle_stats);
                logger.spawn_blocking(
                    "Calculate average clustering coefficient (mid-k)",
                    move || calculate_mid_k_from_stats(&triangle_stats, num_vertices),
                )
            };

            let global_k = {
                let triangle_stats = Arc::clone(&triangle_stats);
                logger.spawn_blocking("Calculate global clustering coefficient", move || {
                    calculate_global_k_from_stats(&triangle_stats, num_triangles)
                })
            };

            let mid_k_component = {
                let triangle_stats = Arc::clone(&triangle_stats);
                let largest_weak_comp = Arc::clone(&largest_weak_comp);
                let graph = Arc::clone(&graph);
                logger.spawn_blocking("Calculate mid-k for largest weak component", move || {
                    calculate_mid_k_from_stats_for_component(
                        &triangle_stats,
                        graph.as_ref(),
                        largest_weak_comp.as_ref(),
                    )
                })
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

            // ── Phase 7: Degree statistics ─────────────────────────────────
            let max_degree_handle = {
                let degrees = Arc::clone(&all_degrees);
                logger.spawn_blocking("Find maximal degree", move || max_degree(degrees.as_ref()))
            };

            let min_degree_handle = {
                let degrees = Arc::clone(&all_degrees);
                logger.spawn_blocking("Find minimal degree", move || min_degree(degrees.as_ref()))
            };

            let mid_degree_handle = {
                let degrees = Arc::clone(&all_degrees);
                logger.spawn_blocking("Compute average degree", move || {
                    mid_degree(degrees.as_ref(), num_vertices)
                })
            };

            let (max_degree_val, min_degree_val, mid_degree_val) =
                tokio::try_join!(max_degree_handle, min_degree_handle, mid_degree_handle).unwrap();

            buffer_for_print_default_info.push((
                "Maximal degree of the graph".to_string(),
                max_degree_val.to_string(),
            ));

            buffer_for_print_default_info.push((
                "Minimal degree of the graph".to_string(),
                min_degree_val.to_string(),
            ));

            buffer_for_print_default_info.push((
                "Average degree of the graph".to_string(),
                format!("{mid_degree_val:.6}"),
            ));

            let mut buffer_for_print_random_removes: Vec<(String, String)> = Vec::new();
            let mut buffer_for_print_removes_of_largest: Vec<(String, String)> = Vec::new();

            // ── Phase 8: Robustness analysis ───────────────────────────────
            log_start(
                &trace_log,
                &file_name,
                start_point,
                "Analyse graph robustness (vertex removal)",
            );
            let lcc_random_removes_handle = {
                let graph = Arc::clone(&graph);
                logger.spawn_blocking(
                    "Robustness: LCC after random vertex removal (10 trials)",
                    move || lcc_after_random_removal(graph.as_ref(), num_vertices, 10),
                )
            };

            let lcc_hub_removes_handle = {
                let graph = Arc::clone(&graph);
                let degrees = degrees_internal(graph.as_ref());
                logger.spawn_blocking("Robustness: LCC after hub vertex removal", move || {
                    lcc_after_hub_removal(graph.as_ref(), num_vertices, &degrees)
                })
            };

            let (lcc_random_removes, lcc_hub_removes) =
                tokio::try_join!(lcc_random_removes_handle, lcc_hub_removes_handle).unwrap();

            for i in 1..=20_u32 {
                let percent = i * 5;
                let fraction_random = lcc_random_removes[&percent];
                let fraction_hub = lcc_hub_removes[&percent];
                buffer_for_print_random_removes
                    .push((format!("{percent}%"), fraction_random.to_string()));
                buffer_for_print_removes_of_largest
                    .push((format!("{percent}%"), fraction_hub.to_string()));
            }

            // ── Finalize: stop animation, print results ────────────────────
            stop_animation.store(true, Ordering::Relaxed);
            let _ = animation_handle.join();

            {
                let start = Instant::now();
                ui::degree_graphic_printing::print_graph(&degree_data);
                logger.log_duration(
                    "Print degree distribution (terminal chart)",
                    start.elapsed(),
                );
            }
            {
                let start = Instant::now();
                ui::degree_graphic_saving_in_png::save_graph_plotters(
                    &degree_data,
                    Some("degree_data1"),
                )
                .expect("Failed to save graph as PNG");
                logger.log_duration(
                    "Save degree distribution as PNG (degree_data1)",
                    start.elapsed(),
                );
            }
            {
                let start = Instant::now();
                ui::degree_graphic_printing::print_graph(&log_degree_data);
                logger.log_duration(
                    "Print log-log degree distribution (terminal chart)",
                    start.elapsed(),
                );
            }
            {
                let start = Instant::now();
                ui::degree_graphic_saving_in_png::save_graph_plotters(
                    &log_degree_data,
                    Some("log_degree_data"),
                )
                .expect("Failed to save graph as PNG");
                logger.log_duration(
                    "Save log-log degree distribution as PNG (log_degree_data)",
                    start.elapsed(),
                );
            }

            logger.log_duration("Total runtime", start_point.elapsed());
            println!("Time: {:.2?}", start_point.elapsed());

            // ── Post-analysis menu ─────────────────────────────────────────
            loop {
                match ui::main_ui::show_main_menu() {
                    ui::main_ui::MainMenuChoice::ViewResults => {
                        let start = Instant::now();
                        print_table(&buffer_for_print_default_info);
                        println!(
                            "\nFraction of largest weak component after remove random n% vertices"
                        );
                        print_table(&buffer_for_print_random_removes);
                        println!(
                            "\nFraction of largest weak component after remove n% vertices with largest degree"
                        );
                        print_table(&buffer_for_print_removes_of_largest);
                        logger.log_duration("Display results table to user", start.elapsed());
                        println!("\nPress Enter to return to menu...");
                        let mut _line = String::new();
                        let _ = std::io::stdin().read_line(&mut _line);
                    }
                    ui::main_ui::MainMenuChoice::InteractiveMode => {
                        let graph = Arc::clone(&graph);
                        log_start(
                            &trace_log,
                            &file_name,
                            start_point,
                            "Interactive landmark session",
                        );
                        interactive_landmarks::interactive_landmark_req::run_landmark_interactive(
                            graph, 10,
                        )
                        .await;
                    }
                    ui::main_ui::MainMenuChoice::Exit => break,
                }
            }
        }
        None => println!("No file selected. Exiting."),
    }
}

/// Format a Unix timestamp as a simple date-time string.
/// Uses day-count from epoch since we don't have chrono in dependencies.
fn format_timestamp(ts: u64) -> String {
    let days = ts / 86400;
    let rem = ts % 86400;
    let hours = rem / 3600;
    let minutes = (rem % 3600) / 60;
    let seconds = rem % 60;
    format!("{days} days since epoch, {hours:02}:{minutes:02}:{seconds:02} UTC")
}
