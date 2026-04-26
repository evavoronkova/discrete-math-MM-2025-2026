use crate::graph::Graph;
use crate::graph::traversal::dfs;
use crate::parser::directed_or_undirected::DirectedOrUndirected;
use std::collections::HashSet;

#[allow(dead_code)]
pub fn calculate_main_graph_information(graph: &Graph) {
    let num_vertices = graph.num_vertices();
    let num_edges = graph.num_edges();
    let density = calculate_density(graph);

    let mut visited = HashSet::new();
    let mut components = 0;
    for vertex in graph.vertices() {
        if !visited.contains(&vertex) {
            dfs(graph, vertex, &mut visited);
            components += 1;
        }
    }
}

pub fn calculate_density(graph: &Graph) -> f64 {
    let n = graph.num_vertices();

    if n < 2 {
        return 0.0;
    }

    let edges = graph.num_edges();

    match graph.kind() {
        DirectedOrUndirected::Undirected => {
            (2.0 * edges as f64) / ((n * (n - 1)) as f64)
        }
        DirectedOrUndirected::Directed => (edges as f64) / ((n * (n - 1)) as f64),
    }
}
