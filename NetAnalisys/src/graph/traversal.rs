use std::collections::{HashSet, HashMap, VecDeque};
use crate::graph::Graph;

pub fn dfs(graph: &Graph, start: u32, visited: &mut HashSet<u32>) {
    if !visited.insert(start) {
        return;
    }

    if let Some(neighbors) = graph.adjacency_list.get(&start) {
        for &neighbor in neighbors {
            dfs(graph, neighbor, visited);
        }
    }
}

pub fn bfs(graph: &Graph, start: u32) -> HashMap<u32, usize> {
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

pub fn bfs_with_filter(
    graph: &Graph,
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

pub fn dfs_for_comps(
    graph: &Graph,
    start: u32,
    visited: &mut HashSet<u32>,
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