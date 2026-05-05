use crate::{
    analysis::connectivity::build_undirected,
    analysis::triangle_counter::{TriangleStats, compute_triangle_stats},
    graph::Graph,
    parser::directed_or_undirected::DirectedOrUndirected,
};
use rustc_hash::FxHashSet as HashSet;

fn choose_2(n: u32) -> f64 {
    if n < 2 {
        0.0
    } else {
        (n as f64 * (n as f64 - 1.0)) / 2.0
    }
}

fn calculate_mid_k_from_stats_internal(stats: &TriangleStats, num_vertices: usize) -> f64 {
    if num_vertices == 0 {
        return 0.0;
    }

    let sum: f64 = (0..stats.degrees.len())
        .map(|v| {
            let triplets = choose_2(stats.degrees[v]);
            if triplets == 0.0 {
                0.0
            } else {
                stats.triangles_per_vertex[v] as f64 / triplets
            }
        })
        .sum();

    sum / num_vertices as f64
}

pub fn calculate_mid_k_from_stats(stats: &TriangleStats, num_vertices: usize) -> f64 {
    calculate_mid_k_from_stats_internal(stats, num_vertices)
}

pub fn calculate_global_k_from_stats(stats: &TriangleStats, num_triangles: u32) -> f64 {
    if stats.triplets_total == 0 {
        return 0.0;
    }

    let triangles = if num_triangles == 0 {
        stats.total_triangles
    } else {
        num_triangles as u64
    };
    (3 * triangles) as f64 / stats.triplets_total as f64
}

pub fn calculate_mid_k_from_stats_for_component(
    stats: &TriangleStats,
    graph: &Graph,
    comp: &HashSet<u32>,
) -> f64 {
    let comp_internal: Vec<u32> = comp
        .iter()
        .filter_map(|&vertex| graph.external_to_internal(vertex))
        .collect();

    if comp_internal.is_empty() {
        return 0.0;
    }

    let sum: f64 = comp_internal
        .iter()
        .map(|&v| {
            let v = v as usize;
            let triplets = choose_2(stats.degrees[v]);
            if triplets == 0.0 {
                0.0
            } else {
                stats.triangles_per_vertex[v] as f64 / triplets
            }
        })
        .sum();

    sum / comp_internal.len() as f64
}

fn calculate_mid_k_with_mask(
    graph: &Graph,
    num_vertices: usize,
    allowed_mask: Option<&[bool]>,
) -> f64 {
    if num_vertices == 0 {
        return 0.0;
    }

    let stats = compute_triangle_stats(graph, allowed_mask);
    calculate_mid_k_from_stats_internal(&stats, num_vertices)
}

pub fn calculate_mid_k(graph: &Graph, num_vertices: usize) -> f64 {
    calculate_mid_k_with_mask(graph, num_vertices, None)
}

pub fn calculate_global_k(graph: &Graph, num_triangles: u32) -> f64 {
    let stats = compute_triangle_stats(graph, None);
    calculate_global_k_from_stats(&stats, num_triangles)
}

pub fn calculate_mid_k_for_weak_component(graph: &Graph, comp: &HashSet<u32>) -> f64 {
    let undirected_graph: Graph;
    let working_graph = match graph.kind() {
        DirectedOrUndirected::Directed => {
            undirected_graph = build_undirected(graph);
            &undirected_graph
        }
        DirectedOrUndirected::Undirected => graph,
    };

    let comp_internal: HashSet<u32> = comp
        .iter()
        .filter_map(|&vertex| working_graph.external_to_internal(vertex))
        .collect();

    let mut allowed_mask = vec![false; working_graph.num_vertices()];
    for &vertex in &comp_internal {
        allowed_mask[vertex as usize] = true;
    }

    calculate_mid_k_with_mask(working_graph, comp_internal.len(), Some(&allowed_mask))
}
