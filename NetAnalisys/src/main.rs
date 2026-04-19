use std::collections::HashMap;

mod graph;
mod parser;

#[allow(dead_code)]
fn calculate_main_graph_information(graph: &graph::Graph) {
    let num_vertices = graph.adjacency_list.len();
    let mut num_edges: usize = graph
        .adjacency_list
        .values()
        .map(|neighbors| neighbors.len())
        .sum();

    if let parser::directed_or_undirected::DirectedOrUndirected::Undirected = graph.graph_type {
        num_edges /= 2;
    }
    let density = calculate_density(graph);

    let mut visited = std::collections::HashSet::new();
    let mut components = 0;
    for &vertex in graph.adjacency_list.keys() {
        if !visited.contains(&vertex) {
            dfs(graph, vertex, &mut visited);
            components += 1;
        }
    }
}
fn calculate_density(graph: &graph::Graph) -> f64 {
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

fn dfs(graph: &graph::Graph, start: u32, visited: &mut std::collections::HashSet<u32>) {
    if !visited.insert(start) {
        return;
    }

    if let Some(neighbors) = graph.adjacency_list.get(&start) {
        for &neighbor in neighbors {
            dfs(graph, neighbor, visited);
        }
    }
}
fn build_undirected(graph: &graph::Graph) -> HashMap<u32, Vec<u32>> {
    let mut undirected = HashMap::new();

    for (&u, neighbors) in &graph.adjacency_list {
        for &v in neighbors {
            undirected.entry(u).or_default().push(v);
            undirected.entry(v).or_default().push(u);
        }
    }

    undirected
}
fn main() {}
