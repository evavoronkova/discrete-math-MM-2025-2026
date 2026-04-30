use crate::{graph::Graph, parser::directed_or_undirected::DirectedOrUndirected};
use rustc_hash::FxHashMap as HashMap;

pub fn all_degrees(graph: &Graph) -> HashMap<u32, u32> {
    let mut degrees = HashMap::default();
    match graph.kind() {
        DirectedOrUndirected::Directed => {
            for (src, targets) in graph.adjacency_entries_internal() {
                let src_external = graph.internal_to_external(src).unwrap();
                *degrees.entry(src_external).or_insert(0) += targets.len() as u32;
                for &tgt in targets {
                    let tgt_external = graph.internal_to_external(tgt).unwrap();
                    *degrees.entry(tgt_external).or_insert(0) += 1;
                }
            }
        }
        DirectedOrUndirected::Undirected => {
            for (src, targets) in graph.adjacency_entries_internal() {
                let src_external = graph.internal_to_external(src).unwrap();
                degrees.insert(src_external, targets.len() as u32);
            }
        }
    }
    degrees
}

fn min_degree_from_degrees(degrees: &HashMap<u32, u32>) -> u32 {
    degrees.values().copied().min().unwrap_or(0)
}

fn min_degree(graph: &Graph) -> u32 {
    let degrees = all_degrees(graph);
    min_degree_from_degrees(&degrees)
}

fn max_degree_from_degrees(degrees: &HashMap<u32, u32>) -> u32 {
    degrees.values().copied().max().unwrap_or(0)
}

fn max_degree(graph: &Graph) -> u32 {
    let degrees = all_degrees(graph);
    max_degree_from_degrees(&degrees)
}

fn mid_degree_from_degrees(degrees: &HashMap<u32, u32>, total_vertices: usize) -> f64 {
    if total_vertices == 0 {
        return 0.0;
    }

    let sum: u32 = degrees.values().sum();
    sum as f64 / total_vertices as f64
}

fn mid_degree(graph: &Graph) -> f64 {
    let degrees = all_degrees(graph);
    mid_degree_from_degrees(&degrees, graph.num_vertices())
}

fn degree_probability_from_degrees(
    degrees: &HashMap<u32, u32>,
    total_vertices: usize,
) -> Vec<(f32, f32)> {
    let mut hashmap: HashMap<u32, f32> = HashMap::default();
    for &degree in degrees.values() {
        *hashmap.entry(degree).or_insert(0.0) += 1.0 / total_vertices as f32;
    }

    let mut data: Vec<(f32, f32)> = hashmap
        .into_iter()
        .map(|(degree, count)| (degree as f32, count))
        .collect();
    data.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    data
}

pub fn degree_probability(graph: &Graph) -> Vec<(f32, f32)> {
    let degrees = all_degrees(graph);
    degree_probability_from_degrees(&degrees, graph.num_vertices())
}

pub fn transform_to_log(data: &Vec<(f32, f32)>) -> Vec<(f32, f32)> {
    data.into_iter()
        .map(|(degree, count)| (f32::log10(*degree), f32::log10(*count)))
        .collect()
}
