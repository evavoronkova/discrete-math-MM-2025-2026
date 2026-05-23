use crate::{
    analysis::connectivity::{
        find_weak_components_masked, fraction_from_component_size, largest_component_size,
    },
    graph::Graph,
    parser::directed_or_undirected::DirectedOrUndirected,
};

use rand::Rng;
use rand::seq::SliceRandom;
use rayon::prelude::*;
use rustc_hash::FxHashMap as HashMap;

/// Sorted by degree descending: Vec of (internal_vertex_id, degree)
fn sorted_by_degree_desc(degrees: &[u32]) -> Vec<(u32, u32)> {
    let mut pairs: Vec<(u32, u32)> = degrees
        .iter()
        .enumerate()
        .map(|(v, &d)| (v as u32, d))
        .collect();
    pairs.par_sort_unstable_by(|a, b| b.1.cmp(&a.1));
    pairs
}

pub fn lcc_after_hub_removal(
    graph: &Graph,
    num_vertices: usize,
    degrees: &[u32],
) -> HashMap<u32, f64> {
    let sorted = sorted_by_degree_desc(degrees);

    (1..=20)
        .into_par_iter()
        .map(|x| {
            let percent = x * 5;
            let num_remove =
                (((percent * num_vertices) as f64 / 100.0).round() as usize).min(num_vertices);

            let mut allowed = vec![true; graph.num_vertices()];
            for &(v_internal, _) in sorted.iter().take(num_remove) {
                allowed[v_internal as usize] = false;
            }

            let comps = find_weak_components_masked(graph, Some(&allowed), num_vertices);
            let largest = largest_component_size(&comps);

            let active_vertices = num_vertices - num_remove;
            let fraction = if active_vertices == 0 {
                0.0
            } else {
                largest as f64 / active_vertices as f64
            };
            (percent as u32, fraction)
        })
        .collect()
}

pub fn lcc_after_random_removal(
    graph: &Graph,
    num_vertices: usize,
    trials: usize,
) -> HashMap<u32, f64> {
    // Internal vertices that exist (all vertices_internal)
    let vertices: Vec<u32> = graph.vertices_internal().collect();

    (1..=20)
        .into_par_iter()
        .map(|x| {
            let percent = x * 5;
            let num_remove =
                (((percent * num_vertices) as f64 / 100.0).round() as usize).min(num_vertices);

            let mut rng = rand::thread_rng();
            let mut total_fraction = 0.0;
            for _ in 0..trials {
                let mut allowed = vec![true; graph.num_vertices()];
                let mut indices: Vec<usize> = (0..vertices.len()).collect();
                indices.shuffle(&mut rng);
                for &idx in indices.iter().take(num_remove) {
                    allowed[vertices[idx] as usize] = false;
                }

                let comps = find_weak_components_masked(graph, Some(&allowed), num_vertices);
                let largest = largest_component_size(&comps);
                let active_vertices = num_vertices - num_remove;
                if active_vertices > 0 {
                    total_fraction += largest as f64 / active_vertices as f64;
                }
            }
            let fraction = total_fraction / trials as f64;
            (percent as u32, fraction)
        })
        .collect()
}
