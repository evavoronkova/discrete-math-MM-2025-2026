use std::collections::{HashMap, HashSet};
use crate::graph::Graph;
use crate::parser::directed_or_undirected::DirectedOrUndirected;
use crate::graph::traversal::dfs_for_comps;

pub fn build_undirected(graph: &Graph) -> Graph {
    let mut undirected_adj = HashMap::new();

    for (&u, neighbors) in &graph.adjacency_list {
        for &v in neighbors {
            undirected_adj.entry(u).or_insert_with(Vec::new).push(v);
            undirected_adj.entry(v).or_insert_with(Vec::new).push(u);
        }
    }

    Graph {
        adjacency_list: undirected_adj,
        graph_type: DirectedOrUndirected::Undirected,
    }
}

pub fn find_weak_components(graph: &Graph) -> Vec<HashSet<u32>> {
    let owned_graph: Graph;
    let graph_ref = match graph.graph_type {
        DirectedOrUndirected::Undirected => graph,
        DirectedOrUndirected::Directed => {
            owned_graph = build_undirected(graph);
            &owned_graph
        }
    };
    let mut visited = HashSet::new();
    let mut components = Vec::new();

    for &vertex in graph_ref.adjacency_list.keys() {
        if !visited.contains(&vertex) {
            let mut comp = HashSet::new();
            dfs_for_comps(graph_ref, vertex, &mut visited, &mut comp);
            components.push(comp);
        }
    }

    components
}

#[allow(clippy::too_many_arguments)]
pub fn strong_connect(
    v: u32,
    adj: &HashMap<u32, Vec<u32>>,
    index_counter: &mut u32,
    indexes: &mut HashMap<u32, u32>,
    lowlinks: &mut HashMap<u32, u32>,
    stack: &mut Vec<u32>,
    on_stack: &mut HashMap<u32, bool>,
    sccs: &mut Vec<HashSet<u32>>,
) {
    *index_counter += 1;
    indexes.insert(v, *index_counter);
    lowlinks.insert(v, *index_counter);
    stack.push(v);
    on_stack.insert(v, true);

    if let Some(neighbors) = adj.get(&v) {
        for &w in neighbors {
            if !indexes.contains_key(&w) {
                strong_connect(
                    w,
                    adj,
                    index_counter,
                    indexes,
                    lowlinks,
                    stack,
                    on_stack,
                    sccs,
                );
                lowlinks.insert(v, lowlinks[&v].min(lowlinks[&w]));
            } else if *on_stack.get(&w).unwrap_or(&false) {
                lowlinks.insert(v, lowlinks[&v].min(indexes[&w]));
            }
        }
    }

    if lowlinks[&v] == indexes[&v] {
        let mut scc = HashSet::new();
        loop {
            let w = stack.pop().unwrap();
            on_stack.insert(w, false);
            scc.insert(w);
            if w == v {
                break;
            }
        }
        sccs.push(scc);
    }
}

pub fn tarjan_scc(graph: &Graph) -> Vec<HashSet<u32>> {
    let mut index_counter = 0;
    let mut indexes = HashMap::new();
    let mut lowlinks = HashMap::new();
    let mut stack = Vec::new();
    let mut on_stack = HashMap::new();
    let mut sccs = Vec::new();

    for &v in graph.adjacency_list.keys() {
        if !indexes.contains_key(&v) {
            strong_connect(
                v,
                &graph.adjacency_list,
                &mut index_counter,
                &mut indexes,
                &mut lowlinks,
                &mut stack,
                &mut on_stack,
                &mut sccs,
            );
        }
    }
    sccs
}

#[allow(clippy::ptr_arg)]
pub fn get_number_of_comps(comps: &mut [HashSet<u32>]) -> u32 {
    comps.len() as u32
}

#[allow(clippy::ptr_arg)]
pub fn fraction_in_largest_component(comps: &mut [HashSet<u32>], num_vertices: usize) -> f64 {
    let max_len = comps.iter().map(|comp| comp.len()).max().unwrap();
    max_len as f64 / num_vertices as f64
}