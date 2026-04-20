use crate::graph;
use std::collections::{HashMap, HashSet};
fn find_triangles(graph: &graph::Graph) -> u32 {
    let mut new_graph: HashMap<u32, HashSet<u32>> = HashMap::new();
    for (&node, neighbours) in graph.adjacency_list.iter() {
        new_graph.insert(node, neighbours.iter().cloned().collect());
    }

    let mut triangles = 0;

    for (&node, neighbours) in new_graph.iter() {
        for &neighbour in neighbours.iter() {
            if node < neighbour {
                let neighbours_node = neighbours;
                let neighbours_neighbour = &new_graph[&neighbour];

                let (small, big) = if neighbours_node.len() < neighbours_neighbour.len() {
                    (neighbours_node, neighbours_neighbour)
                } else {
                    (neighbours_neighbour, neighbours_node)
                };

                for &w in small {
                    if big.contains(&w) {
                        triangles += 1;
                    }
                }
            }
        }
    }

    (triangles / 3) as u32
}
