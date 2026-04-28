use crate::{
    analysis::{cluster_evaluation::get_max_comp, connectivity::{find_weak_components, fraction_in_largest_component, get_largest_comp}, degree::all_degrees},
    graph::Graph,
    parser::directed_or_undirected::DirectedOrUndirected,
};

use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet};

fn remove_vertices(graph: &Graph, to_remove: &HashSet<u32>) -> Graph {
    let mut filtered_graph = Graph::new(graph.kind());
    for (src, targets) in graph.adjacency_entries() {
        if to_remove.contains(&src) {
            continue;
        }

        filtered_graph.add_vertex(src);
        for &target in targets {
            if to_remove.contains(&target) {
                continue;
            }

            match graph.kind() {
                DirectedOrUndirected::Directed => filtered_graph.add_edge(src, target),
                DirectedOrUndirected::Undirected if src < target => {
                    filtered_graph.add_edge(src, target)
                }
                DirectedOrUndirected::Undirected => {}
            }
        }
    }

    filtered_graph
}

fn lcc_after_hub_removal(graph: &Graph) -> HashMap<u32, f64> {
    let mut hashmap: HashMap<u32, f64> = HashMap::new();
    for x in 1..=20 {
        let percent = x * 5;
        let num_vertices_to_delete =
            ((percent * graph.num_vertices()) as f64 / 100.0).round() as u32;
        let num_to_remove = if num_vertices_to_delete > graph.num_vertices() as u32 {
            graph.num_vertices() as u32
        } else {
            num_vertices_to_delete
        };

        let degrees = all_degrees(graph);
        let mut sorted_vertices: Vec<u32> = degrees.keys().cloned().collect();
        sorted_vertices.sort_by(|a, b| degrees[b].cmp(&degrees[a]));

        let mut to_remove: HashSet<u32> = HashSet::new();
        for i in 0..num_to_remove {
            to_remove.insert(sorted_vertices[i as usize]);
        }

        let new_graph = remove_vertices(graph, &to_remove);
        let mut comps = find_weak_components(&new_graph);
        let comp = get_largest_comp(&comps);
        let fraction = fraction_in_largest_component(&comp, new_graph.num_vertices());
        hashmap.insert(percent as u32, fraction);
    }

    hashmap
}

fn lcc_after_random_removal(graph: &Graph, trials: usize) -> HashMap<u32, f64> {
    let mut hashmap: HashMap<u32, f64> = HashMap::new();
    for x in 1..=20 {
        let percent = x * 5;
        let num_vertices_to_delete =
            ((percent * graph.num_vertices()) as f64 / 100.0).round() as u32;
        let num_to_remove = if num_vertices_to_delete > graph.num_vertices() as u32 {
            graph.num_vertices() as u32
        } else {
            num_vertices_to_delete
        };

        let mut rng = rand::thread_rng();
        let mut mid_fraction: f64 = 0.0;
        for _ in 0..trials {
            let mut vertices: Vec<u32> = graph.vertices().collect();
            vertices.shuffle(&mut rng);
            let to_remove: HashSet<u32> =
                vertices.into_iter().take(num_to_remove as usize).collect();

            let new_graph = remove_vertices(graph, &to_remove);
            let mut comps = find_weak_components(&new_graph);
            let comp = get_largest_comp(&comps);
            let fraction = fraction_in_largest_component(&comp, new_graph.num_vertices());
            mid_fraction += fraction;
        }
        mid_fraction /= trials as f64;
        hashmap.insert(percent as u32, mid_fraction);
    }
    hashmap
}
