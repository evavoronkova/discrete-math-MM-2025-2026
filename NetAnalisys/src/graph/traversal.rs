use crate::graph::Graph;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn dfs(graph: &Graph, start: u32, visited: &mut HashSet<u32>) {
    if !visited.insert(start) {
        return;
    }

    for &neighbor in graph.neighbors(start) {
        dfs(graph, neighbor, visited);
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

        for &neighbor in graph.neighbors(node) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                dist.insert(neighbor, current_dist + 1);
                queue.push_back(neighbor);
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

        for &neighbor in graph.neighbors(node) {
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

    dist
}

pub fn bfs_with_parents(graph: &Graph, start: u32) -> HashMap<u32, (usize, Option<u32>)> {
    let mut visited = HashSet::new();
    let mut dist = HashMap::new();
    let mut queue = VecDeque::new();

    visited.insert(start);
    dist.insert(start, (0, None));
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        let (current_dist, _) = dist[&node];

        for &neighbor in graph.neighbors(node) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                dist.insert(neighbor, (current_dist + 1, Some(node)));
                queue.push_back(neighbor);
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
    let mut stack = vec![start];

    while let Some(node) = stack.pop() {
        if !visited.insert(node) {
            continue;
        }

        comp.insert(node);

        for &neighbor in graph.neighbors(node) {
            if !visited.contains(&neighbor) {
                stack.push(neighbor);
            }
        }
    }
}
