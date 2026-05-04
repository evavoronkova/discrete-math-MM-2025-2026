use crate::graph::Graph;
use crate::graph::traversal::bfs_with_filter_internal;
use rand::Rng;
use rand::seq::{IteratorRandom, SliceRandom};
use rayon::prelude::*;
use rustc_hash::FxHashSet as HashSet;
use std::collections::VecDeque;

fn bfs_distances_internal(graph: &Graph, start: u32, allowed_mask: Option<&[bool]>) -> Vec<usize> {
    let n = graph.num_vertices();
    let mut dist = vec![usize::MAX; n];

    if start as usize >= n {
        return dist;
    }

    let mut queue = VecDeque::with_capacity(n.min(1024));
    dist[start as usize] = 0;
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        let current_dist = dist[node as usize];

        for &neighbor in graph.neighbors_internal(node) {
            if allowed_mask.is_some_and(|mask| !mask[neighbor as usize]) {
                continue;
            }

            if dist[neighbor as usize] == usize::MAX {
                dist[neighbor as usize] = current_dist + 1;
                queue.push_back(neighbor);
            }
        }
    }

    dist
}

pub fn approximate_diameter(graph: &Graph, component: Option<&HashSet<u32>>) -> usize {
    let allowed_mask = component.map(|comp| {
        let mut mask = vec![false; graph.num_vertices()];
        for &vertex in comp {
            if let Some(internal) = graph.external_to_internal(vertex) {
                mask[internal as usize] = true;
            }
        }
        mask
    });

    let start = match component {
        Some(comp) => graph
            .external_to_internal(*comp.iter().next().unwrap())
            .unwrap(),
        None => graph.vertices_internal().next().unwrap(),
    };

    let dist = bfs_with_filter_internal(graph, start, allowed_mask.as_deref());

    let (&farthest_node, _) = dist.iter().max_by_key(|&(_, &d)| d).unwrap();

    let dist_from_farthest =
        bfs_with_filter_internal(graph, farthest_node, allowed_mask.as_deref());

    *dist_from_farthest.values().max().unwrap()
}

pub fn random_like_diameter_calculate(
    graph: &Graph,
    component: Option<&HashSet<u32>>,
    iterations: usize,
) -> usize {
    let mut rng = rand::thread_rng();
    let allowed_mask = component.map(|comp| {
        let mut mask = vec![false; graph.num_vertices()];
        for &vertex in comp {
            if let Some(internal) = graph.external_to_internal(vertex) {
                mask[internal as usize] = true;
            }
        }
        mask
    });

    let vertices: Vec<u32> = match component {
        Some(comp) => comp
            .iter()
            .filter_map(|&vertex| graph.external_to_internal(vertex))
            .collect(),
        None => graph.vertices_internal().collect(),
    };

    let mut max_distance = 0;

    for _ in 0..iterations {
        let start = vertices[rng.gen_range(0..vertices.len())];
        let dist = bfs_with_filter_internal(graph, start, allowed_mask.as_deref());

        if let Some(&current_max) = dist.values().max() {
            max_distance = max_distance.max(current_max);
        }
    }

    max_distance
}

pub fn snowball_sampling(
    graph: &Graph,
    component: Option<&HashSet<u32>>,
    sample_size: usize,
) -> HashSet<u32> {
    let mut rng = rand::thread_rng();
    let allowed_mask = component.map(|comp| {
        let mut mask = vec![false; graph.num_vertices()];
        for &vertex in comp {
            if let Some(internal) = graph.external_to_internal(vertex) {
                mask[internal as usize] = true;
            }
        }
        mask
    });
    let vertices: Vec<u32> = match component {
        Some(comp) => comp
            .iter()
            .filter_map(|&vertex| graph.external_to_internal(vertex))
            .collect(),
        None => graph.vertices_internal().collect(),
    };

    if vertices.is_empty() {
        return HashSet::default();
    }

    let start1 = *vertices.choose(&mut rng).unwrap();
    let start2 = *vertices.choose(&mut rng).unwrap();

    let mut queue = VecDeque::new();
    let mut sample = HashSet::default();

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

        for &neighbor in graph.neighbors_internal(node) {
            if sample.len() >= sample_size {
                break;
            }

            let allowed = match &allowed_mask {
                Some(mask) => mask[neighbor as usize],
                None => true,
            };

            if allowed && !sample.contains(&neighbor) {
                sample.insert(neighbor);
                queue.push_back(neighbor);
            }
        }
    }
    sample
        .into_iter()
        .map(|vertex| graph.internal_to_external(vertex).unwrap())
        .collect()
}

pub fn percentile_90_distance(
    graph: &Graph,
    component: Option<&HashSet<u32>>,
    iterations: usize,
) -> usize {
    let vertices: Vec<u32> = match component {
        Some(comp) => comp
            .iter()
            .filter_map(|&v| graph.external_to_internal(v))
            .collect(),
        None => graph.vertices_internal().collect(),
    };

    if vertices.len() < 2 {
        return 0;
    }

    let runs = iterations.min(30).max(1);
    let max_per_run = 50_000;

    let chunks: Vec<Vec<usize>> = (0..runs)
        .into_par_iter()
        .map(|_| {
            let mut rng = rand::thread_rng();
            let start = vertices[rng.gen_range(0..vertices.len())];
            let dist = bfs_distances_internal(graph, start, None);
            dist.into_iter()
                .filter(|&d| d != usize::MAX)
                .choose_multiple(&mut rng, max_per_run)
        })
        .collect();

    let mut distances: Vec<usize> = chunks.into_iter().flatten().collect();
    if distances.is_empty() {
        return 0;
    }

    distances.sort_unstable();
    let idx = ((distances.len() as f64) * 0.9).ceil() as usize;
    distances[idx.min(distances.len() - 1)]
}
