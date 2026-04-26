use crate::{
    analysis::connectivity::build_undirected, analysis::triangle_counter::find_triangles,
    graph::Graph, parser::directed_or_undirected::DirectedOrUndirected,
};
use std::collections::HashSet;

#[allow(dead_code)]
fn calculate_mid_k(graph: &Graph) -> f64 {
    let mut local_k = Vec::new();
    for (_, neighbors) in graph.adjacency_entries() {
        let neighbor_count = neighbors.len();
        let mut actual_edges = 0;
        let max_edges = neighbor_count * (neighbor_count - 1) / 2;
        if max_edges == 0 {
            local_k.push(0.0);
            continue;
        }
        for first_neighbor in neighbors.iter() {
            for second_neighbor in neighbors.iter() {
                if first_neighbor < second_neighbor && graph.has_edge(*first_neighbor, *second_neighbor) {
                    actual_edges += 1;
                }
            }
        }
        local_k.push(actual_edges as f64 / max_edges as f64);
    }
    local_k.iter().sum::<f64>() / graph.num_vertices() as f64
}

#[allow(dead_code)]
fn triplet_counter(graph: &Graph) -> u32 {
    let mut triplets_count = 0;
    for (_, neighbors) in graph.adjacency_entries() {
        let node_degree = neighbors.len();
        triplets_count += (node_degree * (node_degree - 1) / 2) as u32;
    }
    triplets_count
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
    let mut max_comp: HashSet<u32> = HashSet::new();

    for i in comps {
        if max_comp.len() < i.len() {
            max_comp = i.clone();
        }
    }

    max_comp
}
