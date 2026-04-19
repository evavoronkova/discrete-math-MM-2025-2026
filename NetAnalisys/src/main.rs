#![allow(unused)]

use rand::Rng;
use rand::seq::SliceRandom;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    vec,
};

use crate::parser::directed_or_undirected::DirectedOrUndirected;

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

fn build_undirected(graph: &graph::Graph) -> graph::Graph {
    let mut undirected_adj = std::collections::HashMap::new();

    for (&u, neighbors) in &graph.adjacency_list {
        for &v in neighbors {
            undirected_adj.entry(u).or_insert_with(Vec::new).push(v);
            undirected_adj.entry(v).or_insert_with(Vec::new).push(u);
        }
    }

    graph::Graph {
        adjacency_list: undirected_adj,
        graph_type: DirectedOrUndirected::Undirected,
    }
}

fn dfs_for_comps(
    graph: &graph::Graph,
    start: u32,
    visited: &mut std::collections::HashSet<u32>,
    comp: &mut HashSet<u32>,
) {
    if !visited.insert(start) {
        return;
    }

    comp.insert(start);
    if let Some(neighbors) = graph.adjacency_list.get(&start) {
        for &neighbor in neighbors {
            dfs_for_comps(graph, neighbor, visited, comp);
        }
    }
}

fn find_weak_components(
    graph: &graph::Graph,
) -> Vec<HashSet<u32>> {
    let owned_graph: graph::Graph;
    let graph_ref = match graph.graph_type {
        DirectedOrUndirected::Undirected => graph,
        DirectedOrUndirected::Directed => {
            owned_graph = build_undirected(graph);
            &owned_graph
        }
    };
    let mut visited = HashSet::new();
    let mut components = Vec::new();

    for &vertex in graph_ref.adjacency_list.keys() {
        if !visited.contains(&vertex) {
            let mut comp = HashSet::new();
            dfs_for_comps(graph_ref, vertex, &mut visited, &mut comp);
            components.push(comp);
        }
    }

    components
}

fn strong_connect(
    v: u32,
    adj: &HashMap<u32, Vec<u32>>,
    index_counter: &mut u32,
    indexes: &mut HashMap<u32, u32>,
    lowlinks: &mut HashMap<u32, u32>,
    stack: &mut Vec<u32>,
    on_stack: &mut HashMap<u32, bool>,
    sccs: &mut Vec<HashSet<u32>>,
) {
    *index_counter += 1;
    indexes.insert(v, *index_counter);
    lowlinks.insert(v, *index_counter);
    stack.push(v);
    on_stack.insert(v, true);

    if let Some(neighbors) = adj.get(&v) {
        for &w in neighbors {
            if !indexes.contains_key(&w) {
                strong_connect(
                    w,
                    adj,
                    index_counter,
                    indexes,
                    lowlinks,
                    stack,
                    on_stack,
                    sccs,
                );
                lowlinks.insert(v, lowlinks[&v].min(lowlinks[&w]));
            } else if *on_stack.get(&w).unwrap_or(&false) {
                lowlinks.insert(v, lowlinks[&v].min(indexes[&w]));
            }
        }
    }

    if lowlinks[&v] == indexes[&v] {
        let mut scc = HashSet::new();
        loop {
            let w = stack.pop().unwrap();
            on_stack.insert(w, false);
            scc.insert(w);
            if w == v {
                break;
            }
        }
        sccs.push(scc);
    }
}

fn tarjan_scc(graph: &graph::Graph) -> Vec<HashSet<u32>> {
    let mut index_counter = 0;
    let mut indexes = HashMap::new();
    let mut lowlinks = HashMap::new();
    let mut stack = Vec::new();
    let mut on_stack = HashMap::new();
    let mut sccs = Vec::new();

    for &v in graph.adjacency_list.keys() {
        if !indexes.contains_key(&v) {
            strong_connect(
                v,
                &graph.adjacency_list,
                &mut index_counter,
                &mut indexes,
                &mut lowlinks,
                &mut stack,
                &mut on_stack,
                &mut sccs,
            );
        }
    }
    sccs
}


fn get_number_of_comps(comps: &mut Vec<HashSet<u32>>) -> u32 {
    comps.len() as u32
}

fn fraction_in_largest_component(comps: &mut Vec<HashSet<u32>>, num_vertices: usize) -> f64 {
    let max_len = comps.iter().map(|comp| comp.len()).max().unwrap();
    max_len as f64 / num_vertices as f64
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

//work with component

fn bfs_with_filter(
    graph: &graph::Graph,
    start: u32,
    component: Option<&HashSet<u32>>,
) -> HashMap<u32, usize> {
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
                let allowed = match component {
                    Some(comp) => comp.contains(&neighbor),
                    None => true,
                };

                if allowed && !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    dist.insert(neighbor, current_dist + 1);
                    queue.push_back(neighbor);
                }
            }
        }
    }

    dist
}

fn approximate_diameter(graph: &graph::Graph, component: Option<&HashSet<u32>>) -> usize {
    let start = match component {
        Some(comp) => *comp.iter().next().unwrap(),
        None => *graph.adjacency_list.keys().next().unwrap(),
    };

    let dist = bfs_with_filter(graph, start, component);

    let (&farthest_node, _) = dist.iter().max_by_key(|&(_, &d)| d).unwrap();

    let dist_from_farthest = bfs_with_filter(graph, farthest_node, component);

    *dist_from_farthest.values().max().unwrap()
}

fn random_like_diameter_calculate(
    graph: &graph::Graph,
    component: Option<&HashSet<u32>>,
    iterations: usize,
) -> usize {
    let mut rng = rand::thread_rng();

    let vertices: Vec<u32> = match component {
        Some(comp) => comp.iter().cloned().collect(),
        None => graph.adjacency_list.keys().cloned().collect(),
    };

    let mut max_distance = 0;

    for _ in 0..iterations {
        let start = vertices[rng.gen_range(0..vertices.len())];
        let dist = bfs_with_filter(graph, start, component);

        if let Some(&current_max) = dist.values().max() {
            max_distance = max_distance.max(current_max);
        }
    }

    max_distance
}

// use approximate_diameter(graph, Some(snowball_sampling(....))) as u32 to calculate diameter using snowball method
fn snowball_sampling(
    graph: &graph::Graph,
    component: Option<&HashSet<u32>>,
    sample_size: usize,
) -> HashSet<u32> {
    let mut rng = rand::thread_rng();
    let vertices: Vec<u32> = match component {
        Some(comp) => comp.iter().cloned().collect(),
        None => graph.adjacency_list.keys().cloned().collect(),
    };

    if vertices.is_empty() {
        return HashSet::new();
    }

    let start1 = *vertices.choose(&mut rng).unwrap();
    let start2 = *vertices.choose(&mut rng).unwrap();

    let mut queue = VecDeque::new();
    let mut sample = HashSet::new();

    queue.push_back(start1);
    sample.insert(start1);

    if start2 != start1 {
        queue.push_back(start2);
        sample.insert(start2);
    }

    while let Some(node) = queue.pop_front() {
        if sample.len() >= sample_size {
            break;
        }

        if let Some(neighbors) = graph.adjacency_list.get(&node) {
            for &neighbor in neighbors {
                if sample.len() >= sample_size {
                    break;
                }

                let allowed = match component {
                    Some(comp) => comp.contains(&neighbor),
                    None => true,
                };

                if allowed && !sample.contains(&neighbor) {
                    sample.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }
    }
    sample
}

fn main() {}
