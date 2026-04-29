use crate::graph::Graph;
use crate::graph::traversal::{bfs_with_filter, bfs_with_parents};
use rand::Rng;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

pub struct LandmarkBFS {
    landmarks: Vec<u32>,
    spts: HashMap<u32, HashMap<u32, (usize, Option<u32>)>>,
}

impl LandmarkBFS {
    pub fn new(graph: &Graph, num_landmarks: usize) -> Self {
        let landmarks = Self::generate_landmarks(graph, num_landmarks);
        let spts = Self::build_spts(graph, &landmarks);

        Self { landmarks, spts }
    }

    pub fn estimate(&self, graph: &Graph, s: u32, t: u32) -> Option<usize> {
        let nodes = self.collect_subgraph(s, t);

        if !nodes.contains(&s) || !nodes.contains(&t) {
            return None;
        }

        let dist = bfs_with_filter(graph, s, Some(&nodes));
        dist.get(&t).copied()
    }

    fn generate_landmarks(graph: &Graph, num_landmarks: usize) -> Vec<u32> {
        let mut rng = rand::thread_rng();
        let vertices: Vec<u32> = graph.vertices().collect();

        if vertices.is_empty() || num_landmarks == 0 {
            return Vec::new();
        }

        let k = num_landmarks.min(vertices.len());
        let mut chosen = HashSet::default();

        while chosen.len() < k {
            let v = vertices[rng.gen_range(0..vertices.len())];
            chosen.insert(v);
        }

        chosen.into_iter().collect()
    }

    fn build_spts(
        graph: &Graph,
        landmarks: &[u32],
    ) -> HashMap<u32, HashMap<u32, (usize, Option<u32>)>> {
        landmarks
            .iter()
            .map(|&l| (l, bfs_with_parents(graph, l)))
            .collect()
    }

    fn path_to_landmark(&self, v: u32, landmark: u32) -> Vec<u32> {
        let mut path = Vec::new();
        let mut current = v;

        while current != landmark {
            path.push(current);

            if let Some((_, parent)) = self.spts.get(&landmark).and_then(|m| m.get(&current)) {
                if let Some(p) = parent {
                    current = *p;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        path.push(landmark);
        path
    }

    fn collect_subgraph(&self, s: u32, t: u32) -> HashSet<u32> {
        let mut subgraph = HashSet::default();

        for &u in &self.landmarks {
            subgraph.extend(self.path_to_landmark(s, u));
            subgraph.extend(self.path_to_landmark(t, u));
        }

        subgraph
    }
}
