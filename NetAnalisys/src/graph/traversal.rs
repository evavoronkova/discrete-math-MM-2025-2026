use crate::graph::Graph;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
use std::collections::VecDeque;

fn component_mask(graph: &Graph, component: Option<&HashSet<u32>>) -> Option<Vec<bool>> {
    component.map(|comp| {
        let mut mask = vec![false; graph.num_vertices()];
        for &vertex in comp {
            if let Some(internal) = graph.external_to_internal(vertex) {
                mask[internal as usize] = true;
            }
        }
        mask
    })
}

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

pub fn dfs(graph: &Graph, start: u32, visited: &mut HashSet<u32>) {
    let Some(start_internal) = graph.external_to_internal(start) else {
        return;
    };

    let start_external = graph.internal_to_external(start_internal).unwrap();
    if !visited.insert(start_external) {
        return;
    }

    for &neighbor in graph.neighbors_internal(start_internal) {
        let neighbor_external = graph.internal_to_external(neighbor).unwrap();
        dfs(graph, neighbor_external, visited);
    }
}

pub fn bfs(graph: &Graph, start: u32) -> HashMap<u32, usize> {
    let Some(start_internal) = graph.external_to_internal(start) else {
        return HashMap::default();
    };

    bfs_internal(graph, start_internal)
        .into_iter()
        .map(|(vertex, distance)| (graph.internal_to_external(vertex).unwrap(), distance))
        .collect()
}

pub fn bfs_with_filter(
    graph: &Graph,
    start: u32,
    component: Option<&HashSet<u32>>,
) -> HashMap<u32, usize> {
    let Some(start_internal) = graph.external_to_internal(start) else {
        return HashMap::default();
    };

    let allowed_mask = component_mask(graph, component);

    bfs_with_filter_internal(graph, start_internal, allowed_mask.as_deref())
        .into_iter()
        .map(|(vertex, distance)| (graph.internal_to_external(vertex).unwrap(), distance))
        .collect()
}

pub fn bfs_with_parents(graph: &Graph, start: u32) -> HashMap<u32, (usize, Option<u32>)> {
    let Some(start_internal) = graph.external_to_internal(start) else {
        return HashMap::default();
    };

    bfs_with_parents_internal(graph, start_internal)
        .into_iter()
        .map(|(vertex, (distance, parent))| {
            (
                graph.internal_to_external(vertex).unwrap(),
                (
                    distance,
                    parent.map(|parent_vertex| graph.internal_to_external(parent_vertex).unwrap()),
                ),
            )
        })
        .collect()
}

pub fn dfs_for_comps(
    graph: &Graph,
    start: u32,
    visited: &mut HashSet<u32>,
    comp: &mut HashSet<u32>,
) {
    let Some(start_internal) = graph.external_to_internal(start) else {
        return;
    };

    let mut stack = vec![start_internal];

    while let Some(node) = stack.pop() {
        let external = graph.internal_to_external(node).unwrap();

        if !visited.insert(external) {
            continue;
        }

        comp.insert(external);

        for &neighbor in graph.neighbors_internal(node) {
            let neighbor_external = graph.internal_to_external(neighbor).unwrap();
            if !visited.contains(&neighbor_external) {
                stack.push(neighbor);
            }
        }
    }
}
