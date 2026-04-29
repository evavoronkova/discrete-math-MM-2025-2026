use crate::{graph::Graph, parser::directed_or_undirected::DirectedOrUndirected};
use rustc_hash::FxHashMap as HashMap;

pub fn all_degrees(graph: &Graph) -> HashMap<u32, u32> {
    let mut degrees = HashMap::default();
    match graph.kind() {
        DirectedOrUndirected::Directed => {
            for (src, targets) in graph.adjacency_entries() {
                *degrees.entry(src).or_insert(0) += targets.len() as u32;
                for &tgt in targets {
                    *degrees.entry(tgt).or_insert(0) += 1;
                }
            }
        }
        DirectedOrUndirected::Undirected => {
            for (src, targets) in graph.adjacency_entries() {
                degrees.insert(src, targets.len() as u32);
            }
        }
    }
    degrees
}

/*  ПЕРЕДЕЛАТЬ БЛЯТЬ, У НАС НАХУЯ ТО ТРИ РАЗА ПО ВСЕМУ ГРАФУ ПРОХОДИТ, МОЖНО ПРОСТО ПЕРЕДАТЬ СПИСОК ВСЕХ СТЕПЕНЕЙ ИЗ ALL_DEGREES
А ЕЩЕ НАДО ЗАИНЛАЙНИТЬ ИХ
*/
fn min_degree(graph: &Graph) -> u32 {
    all_degrees(graph).values().copied().min().unwrap_or(0)
}

fn max_degree(graph: &Graph) -> u32 {
    all_degrees(graph).values().copied().max().unwrap_or(0)
}

fn mid_degree(graph: &Graph) -> f64 {
    let degrees = all_degrees(graph);
    let total_vertices = graph.num_vertices();
    if total_vertices == 0 {
        return 0.0;
    }
    let sum: u32 = degrees.values().sum();
    sum as f64 / total_vertices as f64
}

pub fn degree_probability(graph: &Graph) -> Vec<(f32, f32)> {
    let degrees = all_degrees(graph);
    let total_vertices = graph.num_vertices();
    let mut hashmap: HashMap<u32, f32> = HashMap::default();
    for (k, v) in &degrees {
        *hashmap.entry(*v).or_insert(0.0) += 1.0 / total_vertices as f32;
    }

    let mut data: Vec<(f32, f32)> = hashmap
        .into_iter()
        .map(|(degree, count)| (degree as f32, count))
        .collect();
    data.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    data
}

pub fn transform_to_log(data: &Vec<(f32, f32)>) -> Vec<(f32, f32)> {
    data.into_iter()
        .map(|(degree, count)| (f32::log10(*degree), f32::log10(*count)))
        .collect()
}
