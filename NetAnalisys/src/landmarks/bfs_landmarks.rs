use crate::graph::Graph;
use crate::graph::traversal::{bfs, bfs_with_parents};
use rand::Rng;
use std::collections::{HashMap, HashSet};

struct LandmarkBFS {
    landmarks: Vec<u32>,
    distances: Vec<HashMap<u32, usize>>,
}

fn build_spts(
    graph: &Graph,
    landmarks: &[u32],
) -> HashMap<u32, HashMap<u32, (usize, Option<u32>)>> {
    landmarks
        .iter()
        .map(|&l| {
            let l = l as u32;
            (l, bfs_with_parents(graph, l))
        })
        .collect()
}

fn path_to_landmark(
    v: u32,
    landmark: u32,
    spts: &HashMap<u32, HashMap<u32, (usize, Option<u32>)>>,
) -> Vec<u32> {
    let mut path = Vec::new();
    let mut current = v;

    while current != landmark {
        path.push(current);
        if let Some((_, parent)) = spts.get(&landmark).and_then(|m| m.get(&current)) {
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
    path.reverse();
    path
}
