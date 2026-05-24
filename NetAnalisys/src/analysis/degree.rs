use crate::{graph::Graph, parser::directed_or_undirected::DirectedOrUndirected};
use rayon::prelude::*;
use rustc_hash::FxHashMap as HashMap;

/// Returns total-degree per vertex, indexed by internal vertex id.
/// For undirected graphs this equals out_degree (which equals in_degree).
/// For directed graphs this is out_degree + in_degree.
pub fn degrees_internal(graph: &Graph) -> Vec<u32> {
    match graph.kind() {
        DirectedOrUndirected::Directed => graph
            .out_degrees()
            .iter()
            .zip(graph.in_degrees().iter())
            .map(|(out, inc)| out + inc)
            .collect(),
        DirectedOrUndirected::Undirected => graph.out_degrees().to_vec(),
    }
}

/// Returns total-degree per vertex keyed by external vertex id.
/// Builds a HashMap for compatibility with existing code.
pub fn all_degrees(graph: &Graph) -> HashMap<u32, u32> {
    let internal = degrees_internal(graph);
    graph
        .vertices_internal()
        .map(|v| {
            let ext = graph.internal_to_external(v).unwrap();
            (ext, internal[v as usize])
        })
        .collect()
}

pub fn min_degree_from_vec(degrees: &[u32]) -> u32 {
    degrees.iter().copied().min().unwrap_or(0)
}

pub fn max_degree_from_vec(degrees: &[u32]) -> u32 {
    degrees.iter().copied().max().unwrap_or(0)
}

pub fn mid_degree_from_vec(degrees: &[u32], total_vertices: usize) -> f64 {
    if total_vertices == 0 {
        return 0.0;
    }
    let sum: u64 = degrees.iter().map(|&d| d as u64).sum();
    sum as f64 / total_vertices as f64
}

pub fn min_degree(degrees: &HashMap<u32, u32>) -> u32 {
    degrees.values().copied().min().unwrap_or(0)
}

pub fn max_degree(degrees: &HashMap<u32, u32>) -> u32 {
    degrees.values().copied().max().unwrap_or(0)
}

pub fn mid_degree(degrees: &HashMap<u32, u32>, total_vertices: usize) -> f64 {
    if total_vertices == 0 {
        return 0.0;
    }
    let sum: u32 = degrees.values().sum();
    sum as f64 / total_vertices as f64
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

/// Build degree probability distribution. Returns a Vec of (degree, probability) pairs.
/// Uses precomputed degree vector for efficiency.
pub fn degree_probability_vec(graph: &Graph) -> Vec<(f32, f32)> {
    let degrees = degrees_internal(graph);
    let n = graph.num_vertices();
    if n == 0 {
        return Vec::new();
    }

    let mut hashmap: HashMap<u32, f32> = HashMap::default();
    for &d in &degrees {
        *hashmap.entry(d).or_insert(0.0) += 1.0 / n as f32;
    }

    let mut data: Vec<(f32, f32)> = hashmap
        .into_iter()
        .map(|(degree, count)| (degree as f32, count))
        .collect();
    data.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    data
}

/// Legacy wrapper that builds the HashMap first, then probability.
pub fn degree_probability(graph: &Graph) -> Vec<(f32, f32)> {
    let degrees = all_degrees(graph);
    degree_probability_from_degrees(&degrees, graph.num_vertices())
}

pub fn transform_to_log(data: &Vec<(f32, f32)>) -> Vec<(f32, f32)> {
    data.iter()
        .filter(|(degree, count)| *degree > 0.0 && *count > 0.0)
        .map(|(degree, count)| (f32::log10(*degree), f32::log10(*count)))
        .collect()
}
