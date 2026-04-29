use crate::{
    analysis::connectivity::build_undirected, analysis::triangle_counter::find_triangles,
    graph::Graph, parser::directed_or_undirected::DirectedOrUndirected,
};
use rayon::prelude::*;
use rustc_hash::FxHashSet as HashSet;

fn calculate_mid_k(graph: &Graph) -> f64 {
    let entries: Vec<_> = graph.adjacency_entries().collect();
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
                    if graph.has_edge(neighbors[i], neighbors[j]) {
                        actual_edges += 1;
                    }
                }
            }

            actual_edges as f64 / max_edges as f64
        })
        .sum();

    sum / graph.num_vertices() as f64
}

fn triplet_counter(graph: &Graph) -> u32 {
    let entries: Vec<_> = graph.adjacency_entries().collect();

    entries
        .par_iter()
        .map(|(_, neighbors)| {
            let n = neighbors.len() as u32;
            n * (n - 1) / 2
        })
        .sum()
}

fn calculate_global_k(graph: &Graph) -> f64 {
    let triangles = find_triangles(graph);
    let triplets = triplet_counter(graph);
    if triplets == 0 {
        return 0.0;
    }
    (3 * triangles) as f64 / triplets as f64
}

fn calculate_mid_k_for_weak_component(graph: &Graph, comps: &Vec<HashSet<u32>>) -> f64 {
    let max_comp = get_max_comp(comps);
    let new_graph = create_graph_on_weak_component(graph, &max_comp);
    calculate_mid_k(&new_graph)
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

    let mut component_graph = Graph::new(DirectedOrUndirected::Undirected);
    for &v in comp {
        component_graph.add_vertex(v);
        for &u in working_graph.neighbors(v) {
            if comp.contains(&u) && v < u {
                component_graph.add_edge(v, u);
            }
        }
    }

    component_graph
}

pub fn get_max_comp(comps: &Vec<HashSet<u32>>) -> HashSet<u32> {
    let mut max_comp: HashSet<u32> = HashSet::default();

    for i in comps {
        if max_comp.len() < i.len() {
            max_comp = i.clone();
        }
    }

    max_comp
}
