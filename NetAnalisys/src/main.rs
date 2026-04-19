use std::{collections::{HashMap, HashSet, VecDeque}, vec};

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

fn bfs(graph: &graph::Graph, start: u32) -> HashMap<u32, usize> {
    let mut visited = HashSet::new();
    let mut dist = HashMap::new();
    let mut queue = VecDeque::new();

    visited.insert(start);
    dist.insert(start, 0);
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        let current_dist = dist[&node];

        if let Some(neighbors) = graph.adjacency_list.get(&node) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    dist.insert(neighbor, current_dist + 1);
                    queue.push_back(neighbor);
                }
            }
        }
    }

    dist
}
fn approximate_diameter(graph: &graph::Graph) -> usize {
    let &start = graph.adjacency_list.keys().next().unwrap();
    let dist = bfs(graph, start);
    let (farthest_node, _) = dist.iter().max_by_key(|&(_, &d)| d).unwrap();
    let dist_from_farthest = bfs(graph, *farthest_node);
    *dist_from_farthest.values().max().unwrap()
}

fn build_undirected(graph: &graph::Graph) -> HashMap<u32, Vec<u32>> {
    let mut undirected: HashMap<u32, Vec<u32>> = HashMap::new();

    for (&u, neighbors) in &graph.adjacency_list {
        for &v in neighbors {
            undirected.entry(u).or_default().push(v);
            undirected.entry(v).or_default().push(u);
        }
    }

    undirected
}


fn dfs_for_comps(graph: &graph::Graph, start: u32, visited: &mut std::collections::HashSet<u32>, comp: &mut Vec<u32>) {
    if !visited.insert(start) {
        return;
    }

    comp.push(start);
    if let Some(neighbors) = graph.adjacency_list.get(&start) {
        for &neighbor in neighbors {
            dfs_for_comps(graph, neighbor, visited, comp);
        }
    }
}

fn find_weak_components(graph: &graph::Graph) -> Vec<Vec<u32>> {
    let mut visited = HashSet::new();
    let mut components = Vec::new();

    for &vertex in graph.adjacency_list.keys() {
        if !visited.contains(&vertex) {
            let mut comp = Vec::new();
            dfs_for_comps(graph, vertex, &mut visited, &mut comp);
            components.push(comp);
        }
    }

    components
}

fn main() {}
