use crate::graph::Graph;
use crate::graph::traversal::dfs;
use crate::parser::directed_or_undirected::DirectedOrUndirected;
use std::collections::HashSet;

#[allow(dead_code)]
pub fn calculate_main_graph_information(graph: &Graph) {
    let num_vertices = graph.adjacency_list.len();
    let mut num_edges: usize = graph
        .adjacency_list
        .values()
        .map(|neighbors| neighbors.len())
        .sum();

    if let DirectedOrUndirected::Undirected = graph.graph_type {
        num_edges /= 2;
    }
    let density = calculate_density(graph);

    let mut visited = HashSet::new();
    let mut components = 0;
    for &vertex in graph.adjacency_list.keys() {
        if !visited.contains(&vertex) {
            dfs(graph, vertex, &mut visited);
            components += 1;
        }
    }
}

pub fn calculate_density(graph: &Graph) -> f64 {
    let n = graph.adjacency_list.len();

    if n < 2 {
        return 0.0;
    }

    let mut edges: usize = graph
        .adjacency_list
        .values()
        .map(|neighbors| neighbors.len())
        .sum();

    use crate::parser::directed_or_undirected::DirectedOrUndirected;

    match graph.graph_type {
        DirectedOrUndirected::Undirected => {
            edges /= 2;
            (2.0 * edges as f64) / ((n * (n - 1)) as f64)
        }
        DirectedOrUndirected::Directed => (edges as f64) / ((n * (n - 1)) as f64),
    }
}
