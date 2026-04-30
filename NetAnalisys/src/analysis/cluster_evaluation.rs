use crate::{
    analysis::connectivity::{build_undirected, get_largest_comp},
    analysis::triangle_counter::find_triangles,
    graph::Graph,
    parser::directed_or_undirected::DirectedOrUndirected,
};
use rayon::prelude::*;
use rustc_hash::FxHashSet as HashSet;

pub fn calculate_mid_k(graph: &Graph, num_vertices: usize) -> f64 {
    let entries: Vec<_> = graph.adjacency_entries_internal().collect();
    let sum: f64 = entries
        .par_iter()
        .map(|(_, neighbors)| {
            let n = neighbors.len();
            let max_edges = n * (n - 1) / 2;

            if max_edges == 0 {
                return 0.0;
            }

            let mut actual_edges = 0;

            for i in 0..n {
                for j in (i + 1)..n {
                    if graph.has_edge_internal(neighbors[i], neighbors[j]) {
                        actual_edges += 1;
                    }
                }
            }

            actual_edges as f64 / max_edges as f64
        })
        .sum();

    sum / num_vertices as f64
}

fn triplet_counter(graph: &Graph) -> u32 {
    let entries: Vec<_> = graph.adjacency_entries_internal().collect();

    entries
        .par_iter()
        .map(|(_, neighbors)| {
            let n = neighbors.len() as u32;
            n * (n - 1) / 2
        })
        .sum()
}

pub fn calculate_global_k(graph: &Graph, num_triangles: u32) -> f64 {
    let triplets = triplet_counter(graph);
    if triplets == 0 {
        return 0.0;
    }
    (3 * num_triangles) as f64 / triplets as f64
}

pub fn calculate_mid_k_for_weak_component(graph: &Graph, comp: &HashSet<u32>) -> f64 {
    let new_graph = create_graph_on_weak_component(graph, comp);
    let num_vertices = new_graph.num_vertices();
    calculate_mid_k(&new_graph, num_vertices)
}

fn create_graph_on_weak_component(graph: &Graph, comp: &HashSet<u32>) -> Graph {
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

    let mut component_graph = Graph::new(DirectedOrUndirected::Undirected);
    for &v in &comp_internal {
        let v_external = working_graph.internal_to_external(v).unwrap();
        component_graph.add_vertex(v_external);
        for &u in working_graph.neighbors_internal(v) {
            if comp_internal.contains(&u) && v < u {
                let u_external = working_graph.internal_to_external(u).unwrap();
                component_graph.add_edge(v_external, u_external);
            }
        }
    }

    component_graph
}
