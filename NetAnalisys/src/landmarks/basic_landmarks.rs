use crate::graph::Graph;
use crate::graph::traversal::bfs_internal;
use crate::landmarks::LandmarkStrategy;
use crate::parser::directed_or_undirected::DirectedOrUndirected;
use rand::Rng;
use rayon::prelude::*;
use rustc_hash::FxHashSet as HashSet;
use std::collections::VecDeque;

pub struct LandmarkBasic {
    landmarks: Vec<u32>,
    distances: Vec<Vec<u32>>,
    curr_strat: LandmarkStrategy,
}

impl LandmarkBasic {
    pub fn new(graph: &Graph, num_landmarks: usize, strategy: LandmarkStrategy) -> Option<Self> {
        let n = graph.num_vertices();
        if n == 0 || num_landmarks == 0 {
            return None;
        }

        let k = num_landmarks.min(n);
        let curr_strat = strategy;
        let landmarks: Vec<u32> = match strategy {
            LandmarkStrategy::Random => Self::random_selection(graph, k),
            LandmarkStrategy::HighestDegree => Self::highest_degree_selection(graph, k),
            LandmarkStrategy::Coverage => Self::coverage_selection(graph, k),
        };

        if landmarks.is_empty() {
            return None;
        }

        let distances: Vec<Vec<u32>> = landmarks
            .iter()
            .map(|&l| {
                let dist_map = bfs_internal(graph, l);
                let mut dists = vec![u32::MAX; n];
                for (vertex, d) in dist_map {
                    dists[vertex as usize] = d as u32;
                }
                dists
            })
            .collect();

        Some(Self {
            landmarks,
            distances,
            curr_strat,
        })
    }

    pub fn curr_strategy(self) -> LandmarkStrategy {
        self.curr_strat
    }

    fn random_selection(graph: &Graph, k: usize) -> Vec<u32> {
        let mut rng = rand::thread_rng();
        let mut chosen = HashSet::default();
        let vertices: Vec<u32> = graph.vertices_internal().collect();

        while chosen.len() < k {
            let v = vertices[rng.gen_range(0..vertices.len())];
            chosen.insert(v);
        }

        chosen.into_iter().collect()
    }

    fn highest_degree_selection(graph: &Graph, k: usize) -> Vec<u32> {
        let n = graph.num_vertices();
        let mut deg: Vec<(u32, u32)> = Vec::with_capacity(n);
        let undirected = graph.kind() == DirectedOrUndirected::Undirected;

        for v in graph.vertices_internal() {
            let out_d = graph.out_degree(v);
            let total_d = if undirected {
                out_d
            } else {
                out_d + graph.in_degree(v)
            };
            deg.push((v, total_d));
        }

        if deg.len() <= k {
            return deg.into_par_iter().map(|(v, _)| v).collect();
        }

        deg.select_nth_unstable_by(k, |a, b| b.1.cmp(&a.1));
        deg.truncate(k);
        deg.into_iter().map(|(v, _)| v).collect()
    }

    fn coverage_selection(graph: &Graph, k: usize) -> Vec<u32> {
        let n = graph.num_vertices();
        let mut rng = rand::thread_rng();
        let mut selected: Vec<u32> = Vec::with_capacity(k);
        let mut selected_mask = vec![false; n];

        let mut min_dist = vec![usize::MAX; n];

        let first = rng.gen_range(0..n) as u32;
        selected_mask[first as usize] = true;
        selected.push(first);

        let dists = Self::bfs_all(graph, first);
        min_dist.par_iter_mut().enumerate().for_each(|(i, v)| {
            *v = dists[i];
        });

        for _ in 1..k {
            let farthest = min_dist
                .par_iter()
                .enumerate()
                .filter(|&(i, &d)| d != usize::MAX && d != 0 && !selected_mask[i])
                .max_by_key(|&(_, &d)| d)
                .map(|(i, _)| i as u32);

            let farthest = match farthest {
                Some(v) => v,
                None => break,
            };

            selected_mask[farthest as usize] = true;
            selected.push(farthest);

            let dists = Self::bfs_all(graph, farthest);
            min_dist.par_iter_mut().enumerate().for_each(|(i, v)| {
                if dists[i] < *v {
                    *v = dists[i];
                }
            });
        }

        selected
    }

    fn bfs_all(graph: &Graph, start: u32) -> Vec<usize> {
        let n = graph.num_vertices();
        let mut dist = vec![usize::MAX; n];
        let mut queue = VecDeque::new();

        dist[start as usize] = 0;
        queue.push_back(start);

        while let Some(node) = queue.pop_front() {
            let cur = dist[node as usize];
            for &nbr in graph.neighbors_internal(node) {
                if dist[nbr as usize] == usize::MAX {
                    dist[nbr as usize] = cur + 1;
                    queue.push_back(nbr);
                }
            }
        }

        dist
    }

    /// Estimate distance between two vertices (external IDs).
    /// Uses graph's id mapping for conversion.
    pub fn estimate(&self, graph: &Graph, s: u32, t: u32) -> Option<usize> {
        let s = graph.external_to_internal(s)? as usize;
        let t = graph.external_to_internal(t)? as usize;
        let mut best: Option<usize> = None;

        for dists in &self.distances {
            let ds = dists[s];
            let dt = dists[t];
            if ds != u32::MAX && dt != u32::MAX {
                let cand = ds as usize + dt as usize;
                best = Some(best.map_or(cand, |b| b.min(cand)));
            }
        }

        best
    }
}
