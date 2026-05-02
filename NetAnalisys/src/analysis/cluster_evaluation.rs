use crate::{
    analysis::connectivity::build_undirected,
    analysis::triangle_counter::compute_triangle_stats,
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

fn calculate_mid_k_with_mask(graph: &Graph, num_vertices: usize, allowed_mask: Option<&[bool]>) -> f64 {
    if num_vertices == 0 {
        return 0.0;
    }

    // Оба коэффициента теперь считаются из одного и того же быстрого движка
    // перечисления треугольников, без отдельного двойного обхода по парам соседей.
    let stats = compute_triangle_stats(graph, allowed_mask);
    let sum: f64 = graph
        .vertices_internal()
        .filter(|&v| allowed_mask.is_none_or(|mask| mask[v as usize]))
        .map(|v| {
            let triplets = choose_2(stats.degrees[v as usize]);
            if triplets == 0.0 {
                0.0
            } else {
                stats.triangles_per_vertex[v as usize] as f64 / triplets
            }
        })
        .sum();

    sum / num_vertices as f64
}

pub fn calculate_mid_k(graph: &Graph, num_vertices: usize) -> f64 {
    calculate_mid_k_with_mask(graph, num_vertices, None)
}

pub fn calculate_global_k(graph: &Graph, num_triangles: u32) -> f64 {
    let stats = compute_triangle_stats(graph, None);
    if stats.triplets_total == 0 {
        return 0.0;
    }

    // Сигнатуру оставляем как есть, чтобы не трогать остальной код.
    let triangles = if num_triangles == 0 {
        stats.total_triangles
    } else {
        num_triangles as u64
    };
    (3 * triangles) as f64 / stats.triplets_total as f64
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
