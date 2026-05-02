use crate::{graph::Graph, parser::directed_or_undirected::DirectedOrUndirected};
use rayon::prelude::*;
use rustc_hash::FxHashMap as HashMap;

pub fn all_degrees(graph: &Graph) -> HashMap<u32, u32> {
    let mut degrees = HashMap::default();
    match graph.kind() {
        DirectedOrUndirected::Directed => {
            let entries = graph.adjacency_entries_internal().collect::<Vec<_>>();
            let mut local_degrees: Vec<(u32, u32)> = entries
                .par_iter()
                .flat_map(|(src, targets)| {
                    let src_external = graph.internal_to_external(*src).unwrap();
                    let mut result = vec![(src_external, targets.len() as u32)];
                    for &tgt in *targets {
                        let tgt_external = graph.internal_to_external(tgt).unwrap();
                        result.push((tgt_external, 1));
                    }
                    result
                })
                .collect();
            for (vertex, delta) in local_degrees {
                *degrees.entry(vertex).or_insert(0) += delta;
            }
        }
        DirectedOrUndirected::Undirected => {
            let entries = graph.adjacency_entries_internal().collect::<Vec<_>>();
            let local_degrees: Vec<(u32, u32)> = entries
                .par_iter()
                .flat_map(|(src, targets)| {
                    let src_external = graph.internal_to_external(*src).unwrap();
                    vec![(src_external, targets.len() as u32)]
                })
                .collect();
            for (vertex, degree) in local_degrees {
                *degrees.entry(vertex).or_insert(0) += degree;
            }
        }
    }
    degrees
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

pub fn degree_probability(graph: &Graph) -> Vec<(f32, f32)> {
    let degrees = all_degrees(graph);
    degree_probability_from_degrees(&degrees, graph.num_vertices())
}

pub fn transform_to_log(data: &Vec<(f32, f32)>) -> Vec<(f32, f32)> {
    data.into_iter()
        .map(|(degree, count)| (f32::log10(*degree), f32::log10(*count)))
        .collect()
}
