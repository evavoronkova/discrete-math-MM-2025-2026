use crate::{
    analysis::{
        connectivity::{find_weak_components, fraction_from_component_size, largest_component_size},
        degree::all_degrees,
    },
    graph::Graph,
    parser::directed_or_undirected::DirectedOrUndirected,
};

use rand::seq::SliceRandom;
use rayon::prelude::*;
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
    let num_vertices = graph.num_vertices();
    let degrees = all_degrees(graph);
    let mut sorted_vertices: Vec<u32> = degrees.keys().cloned().collect();
    sorted_vertices.sort_by(|a, b| degrees[b].cmp(&degrees[a]));

    (1..=20)
        .into_par_iter()
        .map(|x| {
            let percent = x * 5;

            let num_vertices_to_delete = ((percent * num_vertices) as f64 / 100.0).round() as u32;

            let num_to_remove = num_vertices_to_delete.min(num_vertices as u32);

            let to_remove: HashSet<u32> = sorted_vertices
                .iter()
                .take(num_to_remove as usize)
                .cloned()
                .collect();

            let new_graph = remove_vertices(graph, &to_remove);
            let comps = find_weak_components(&new_graph);
            let largest_comp_size = largest_component_size(&comps);
            let fraction = fraction_from_component_size(largest_comp_size, new_graph.num_vertices());

            (percent as u32, fraction)
        })
        .collect()
}

fn lcc_after_random_removal(graph: &Graph, trials: usize) -> HashMap<u32, f64> {
    let num_vertices = graph.num_vertices();
    let vertices: Vec<u32> = graph.vertices().collect();

    (1..=20)
        .into_par_iter()
        .map(|x| {
            let percent = x * 5;
            let num_vertices_to_delete = ((percent * num_vertices) as f64 / 100.0).round() as u32;
            let num_to_remove = num_vertices_to_delete.min(num_vertices as u32);

            let mut rng = rand::thread_rng();
            let mut mid_fraction: f64 = 0.0;
            for _ in 0..trials {
                let mut vertices = vertices.clone();
                vertices.shuffle(&mut rng);
                let to_remove: HashSet<u32> =
                    vertices.into_iter().take(num_to_remove as usize).collect();

                let new_graph = remove_vertices(graph, &to_remove);
                let comps = find_weak_components(&new_graph);
                let largest_comp_size = largest_component_size(&comps);
                let fraction =
                    fraction_from_component_size(largest_comp_size, new_graph.num_vertices());
                mid_fraction += fraction;
            }
            mid_fraction /= trials as f64;
            (percent as u32, mid_fraction)
        })
        .collect()
}
