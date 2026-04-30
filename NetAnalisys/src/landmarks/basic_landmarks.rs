use crate::graph::Graph;
use crate::graph::traversal::bfs_internal;
use rand::Rng;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

pub struct LandmarkBasic {
    landmarks: Vec<u32>,
    distances: Vec<HashMap<u32, usize>>,
    external_to_internal: HashMap<u32, u32>,
}

impl LandmarkBasic {
    pub fn new(graph: &Graph, num_landmarks: usize) -> Option<Self> {
        let vertices: Vec<u32> = graph.vertices_internal().collect();
        if vertices.is_empty() || num_landmarks == 0 {
            return None;
        }

        let k = num_landmarks.min(vertices.len());
        let mut rng = rand::thread_rng();
        let mut chosen = HashSet::default();

        while chosen.len() < k {
            let v = vertices[rng.gen_range(0..vertices.len())];
            chosen.insert(v);
        }

        let landmarks: Vec<u32> = chosen.into_iter().collect();
        let distances = landmarks.iter().map(|&l| bfs_internal(graph, l)).collect();
        let external_to_internal = graph
            .vertices_internal()
            .map(|internal| (graph.internal_to_external(internal).unwrap(), internal))
            .collect();

        Some(Self {
            landmarks,
            distances,
            external_to_internal,
        })
    }

    pub fn estimate(&self, s: u32, t: u32) -> Option<usize> {
        let s = *self.external_to_internal.get(&s)?;
        let t = *self.external_to_internal.get(&t)?;
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
