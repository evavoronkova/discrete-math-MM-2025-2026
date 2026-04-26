use crate::graph::traversal::bfs;
use crate::graph::Graph;
use rand::Rng;
use std::collections::{HashMap, HashSet};

pub struct LandmarkBasic {
    landmarks: Vec<u32>,
    distances: Vec<HashMap<u32, usize>>,
}

impl LandmarkBasic {
    pub fn new(graph: &Graph, num_landmarks: usize) -> Option<Self> {
        let vertices: Vec<u32> = graph.adjacency_list.keys().cloned().collect();
        if vertices.is_empty() || num_landmarks == 0 {
            return None;
        }

        let k = num_landmarks.min(vertices.len());
        let mut rng = rand::thread_rng();
        let mut chosen = HashSet::new();

        while chosen.len() < k {
            let v = vertices[rng.gen_range(0..vertices.len())];
            chosen.insert(v);
        }

        let landmarks: Vec<u32> = chosen.into_iter().collect();
        let distances = landmarks.iter().map(|&l| bfs(graph, l)).collect();

        Some(Self { landmarks, distances })
    }

    pub fn estimate(&self, s: u32, t: u32) -> Option<usize> {
        let mut best: Option<usize> = None;

        for dist_map in &self.distances {
            if let (Some(&ds), Some(&dt)) = (dist_map.get(&s), dist_map.get(&t)) {
                let cand = ds.saturating_add(dt);
                best = Some(best.map_or(cand, |b| b.min(cand)));
            }
        }

        best
    }
}