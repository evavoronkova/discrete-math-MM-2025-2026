use crate::{
    analysis::connectivity::build_undirected, graph::Graph,
    parser::directed_or_undirected::DirectedOrUndirected,
};
use std::collections::HashMap;

fn all_degrees(graph: &Graph) -> HashMap<u32, u32> {
    let mut degrees = HashMap::new();
    match graph.graph_type {
        DirectedOrUndirected::Directed => {
            for (&src, targets) in &graph.adjacency_list {
                *degrees.entry(src).or_insert(0) += targets.len() as u32;
                for &tgt in targets {
                    *degrees.entry(tgt).or_insert(0) += 1;
                }
            }
        }
        DirectedOrUndirected::Undirected => {
            for (&src, targets) in &graph.adjacency_list {
                degrees.insert(src, targets.len() as u32);
            }
        }
    }
    degrees
}

fn min_degree(graph: &Graph) -> u32 {
    all_degrees(graph).values().copied().min().unwrap_or(0)
}

fn max_degree(graph: &Graph) -> u32 {
    all_degrees(graph).values().copied().max().unwrap_or(0)
}

fn mid_degree(graph: &Graph) -> f64 {
    let degrees = all_degrees(graph);
    let total_vertices = graph.num_vertices();
    if total_vertices == 0 {
        return 0.0;
    }
    let sum: u32 = degrees.values().sum();
    sum as f64 / total_vertices as f64
}
