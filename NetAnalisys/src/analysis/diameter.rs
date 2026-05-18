use crate::graph::Graph;
use rand::Rng;
use rand::seq::{IteratorRandom, SliceRandom};
use rayon::prelude::*;
use rustc_hash::FxHashSet as HashSet;
use std::collections::VecDeque;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::task;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DiameterMethod {
    DoubleBfs,
    RandomLike,
    Snowball,
    All,
}

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

fn spawn_blocking_diameter_logged<T, F>(
    perf_log: &Arc<Mutex<std::fs::File>>,
    label: &'static str,
    job: F,
) -> tokio::task::JoinHandle<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    let log = Arc::clone(perf_log);
    task::spawn_blocking(move || {
        let start = Instant::now();
        let result = job();
        let mut file = log.lock().unwrap();
        let _ = writeln!(file, "{label}\t{elapsed:.6?}", elapsed = start.elapsed());
        result
    })
}

pub async fn count_diameters(
    graph: Arc<Graph>,
    component: Option<Arc<HashSet<u32>>>,
    perf_log: Arc<Mutex<std::fs::File>>,
) -> Vec<usize> {
    let graph_1 = Arc::clone(&graph);
    let graph_2 = Arc::clone(&graph);
    let graph_3 = Arc::clone(&graph);

    let component_1 = component.as_ref().map(Arc::clone);
    let component_2 = component.as_ref().map(Arc::clone);
    let component_3 = component.as_ref().map(Arc::clone);

    let log_1 = Arc::clone(&perf_log);
    let log_2 = Arc::clone(&perf_log);
    let log_3 = Arc::clone(&perf_log);

    let (diameter_on_double_bfs, diameter_on_random, diameter_on_snowball_sampling) =
        tokio::try_join!(
            spawn_blocking_diameter_logged(&log_1, "approximate_diameter", move || {
                approximate_diameter(&graph_1, component_1.as_deref())
            }),
            spawn_blocking_diameter_logged(&log_2, "random_like_diameter", move || {
                random_like_diameter_calculate(&graph_2, component_2.as_deref(), 500)
            }),
            spawn_blocking_diameter_logged(&log_3, "snowball_sampling", move || {
                snowball_sampling(&graph_3, component_3.as_deref(), 1000)
            }),
        )
        .unwrap();

    vec![
        diameter_on_double_bfs,
        diameter_on_random,
        diameter_on_snowball_sampling,
    ]
}

pub(crate) fn approximate_diameter(graph: &Graph, component: Option<&HashSet<u32>>) -> usize {
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
        Some(comp) => match comp
            .iter()
            .next()
            .and_then(|&v| graph.external_to_internal(v))
        {
            Some(v) => v,
            None => return 0,
        },
        None => match graph.vertices_internal().next() {
            Some(v) => v,
            None => return 0,
        },
    };

    let dist = bfs_distances_internal(graph, start, allowed_mask.as_deref());

    let farthest_node = match dist
        .iter()
        .enumerate()
        .filter(|&(_, &d)| d != usize::MAX)
        .max_by_key(|&(_, &d)| d)
        .map(|(i, _)| i as u32)
    {
        Some(v) => v,
        None => return 0,
    };

    let dist_from_farthest = bfs_distances_internal(graph, farthest_node, allowed_mask.as_deref());

    dist_from_farthest
        .iter()
        .filter(|&&d| d != usize::MAX)
        .max()
        .copied()
        .unwrap_or(0)
}

pub(crate) fn random_like_diameter_calculate(
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
        if vertices.is_empty() {
            break;
        }
        let start = vertices[rng.gen_range(0..vertices.len())];
        let dist = bfs_distances_internal(graph, start, allowed_mask.as_deref());

        if let Some(&current_max) = dist.iter().filter(|&&d| d != usize::MAX).max() {
            max_distance = max_distance.max(current_max);
        }
    }

    max_distance
}

pub(crate) fn snowball_sampling(
    graph: &Graph,
    component: Option<&HashSet<u32>>,
    sample_size: usize,
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

    if vertices.is_empty() {
        return 0;
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
    let comp = sample
        .into_iter()
        .map(|vertex| graph.internal_to_external(vertex).unwrap())
        .collect();

    approximate_diameter(graph, Some(&comp))
}

pub fn percentile_90_distance(
    graph: &Graph,
    component: Option<&HashSet<u32>>,
    iterations: usize,
) -> usize {
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
            .filter_map(|&v| graph.external_to_internal(v))
            .collect(),
        None => graph.vertices_internal().collect(),
    };

    if vertices.len() < 2 {
        return 0;
    }

    let runs = iterations.clamp(1, 30);
    let max_per_run = 50_000;

    let chunks: Vec<Vec<usize>> = (0..runs)
        .into_par_iter()
        .map(|_| {
            let mut rng = rand::thread_rng();
            let start = vertices[rng.gen_range(0..vertices.len())];
            let dist = bfs_distances_internal(graph, start, allowed_mask.as_deref());
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
