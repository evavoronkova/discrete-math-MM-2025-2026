use crate::graph::Graph;
use rustc_hash::FxHashMap as HashMap;
use std::collections::VecDeque;

pub fn bfs_internal(graph: &Graph, start: u32) -> HashMap<u32, usize> {
    let mut visited = vec![false; graph.num_vertices()];
    let mut dist = vec![None; graph.num_vertices()];
    let mut queue = VecDeque::new();

    if start as usize >= graph.num_vertices() {
        return HashMap::default();
    }

    visited[start as usize] = true;
    dist[start as usize] = Some(0);
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        let current_dist = dist[node as usize].unwrap();

        for &neighbor in graph.neighbors_internal(node) {
            if !visited[neighbor as usize] {
                visited[neighbor as usize] = true;
                dist[neighbor as usize] = Some(current_dist + 1);
                queue.push_back(neighbor);
            }
        }
    }

    dist.into_iter()
        .enumerate()
        .filter_map(|(vertex, distance)| distance.map(|d| (vertex as u32, d)))
        .collect()
}

pub fn bfs_with_filter_internal(
    graph: &Graph,
    start: u32,
    allowed_mask: Option<&[bool]>,
) -> HashMap<u32, usize> {
    let mut visited = vec![false; graph.num_vertices()];
    let mut dist = vec![None; graph.num_vertices()];
    let mut queue = VecDeque::new();

    if start as usize >= graph.num_vertices() {
        return HashMap::default();
    }

    visited[start as usize] = true;
    dist[start as usize] = Some(0);
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        let current_dist = dist[node as usize].unwrap();

        for &neighbor in graph.neighbors_internal(node) {
            let allowed = match allowed_mask {
                Some(mask) => mask[neighbor as usize],
                None => true,
            };

            if allowed && !visited[neighbor as usize] {
                visited[neighbor as usize] = true;
                dist[neighbor as usize] = Some(current_dist + 1);
                queue.push_back(neighbor);
            }
        }
    }

    dist.into_iter()
        .enumerate()
        .filter_map(|(vertex, distance)| distance.map(|d| (vertex as u32, d)))
        .collect()
}

pub fn bfs_with_parents_internal(graph: &Graph, start: u32) -> HashMap<u32, (usize, Option<u32>)> {
    let mut visited = vec![false; graph.num_vertices()];
    let mut dist = vec![None; graph.num_vertices()];
    let mut parent = vec![None; graph.num_vertices()];
    let mut queue = VecDeque::new();

    if start as usize >= graph.num_vertices() {
        return HashMap::default();
    }

    visited[start as usize] = true;
    dist[start as usize] = Some(0);
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        let current_dist = dist[node as usize].unwrap();

        for &neighbor in graph.neighbors_internal(node) {
            if !visited[neighbor as usize] {
                visited[neighbor as usize] = true;
                dist[neighbor as usize] = Some(current_dist + 1);
                parent[neighbor as usize] = Some(node);
                queue.push_back(neighbor);
            }
        }
    }

    dist.into_iter()
        .enumerate()
        .filter_map(|(vertex, distance)| distance.map(|d| (vertex as u32, (d, parent[vertex]))))
        .collect()
}
