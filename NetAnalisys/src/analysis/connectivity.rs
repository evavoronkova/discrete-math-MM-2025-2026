use crate::graph::Graph;
use crate::graph::traversal::dfs_for_comps;
use crate::parser::directed_or_undirected::DirectedOrUndirected;
use std::collections::{HashMap, HashSet};

pub fn build_undirected(graph: &Graph) -> Graph {
    let mut undirected_graph = Graph::new(DirectedOrUndirected::Undirected);

    for (u, neighbors) in graph.adjacency_entries() {
        undirected_graph.add_vertex(u);
        for &v in neighbors {
            undirected_graph.add_edge(u, v);
        }
    }

    undirected_graph
}

pub fn find_weak_components(graph: &Graph) -> Vec<HashSet<u32>> {
    let owned_graph: Graph;
    let graph_ref = match graph.kind() {
        DirectedOrUndirected::Undirected => graph,
        DirectedOrUndirected::Directed => {
            owned_graph = build_undirected(graph);
            &owned_graph
        }
    };
    let mut visited = HashSet::new();
    let mut components = Vec::new();

    for vertex in graph_ref.vertices() {
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
    graph: &Graph,
    index_counter: &mut u32,
    indexes: &mut HashMap<u32, u32>,
    lowlinks: &mut HashMap<u32, u32>,
    stack: &mut Vec<u32>,
    on_stack: &mut HashMap<u32, bool>,
    sccs: &mut Vec<HashSet<u32>>,
) {
    struct Frame {
        vertex: u32,
        next_neighbor_idx: usize,
        parent: Option<u32>,
    }

    *index_counter += 1;
    indexes.insert(v, *index_counter);
    lowlinks.insert(v, *index_counter);
    stack.push(v);
    on_stack.insert(v, true);

    let mut frames = vec![Frame {
        vertex: v,
        next_neighbor_idx: 0,
        parent: None,
    }];

    while let Some(mut frame) = frames.pop() {
        let neighbors = graph.neighbors(frame.vertex);

        if frame.next_neighbor_idx < neighbors.len() {
            let current_vertex = frame.vertex;
            let neighbor = neighbors[frame.next_neighbor_idx];
            frame.next_neighbor_idx += 1;
            frames.push(frame);

            if !indexes.contains_key(&neighbor) {
                *index_counter += 1;
                indexes.insert(neighbor, *index_counter);
                lowlinks.insert(neighbor, *index_counter);
                stack.push(neighbor);
                on_stack.insert(neighbor, true);

                frames.push(Frame {
                    vertex: neighbor,
                    next_neighbor_idx: 0,
                    parent: Some(current_vertex),
                });
            } else if *on_stack.get(&neighbor).unwrap_or(&false) {
                let new_lowlink = lowlinks[&current_vertex].min(indexes[&neighbor]);
                lowlinks.insert(current_vertex, new_lowlink);
            }

            continue;
        }

        if let Some(parent) = frame.parent {
            let parent_lowlink = lowlinks[&parent].min(lowlinks[&frame.vertex]);
            lowlinks.insert(parent, parent_lowlink);
        }

        if lowlinks[&frame.vertex] == indexes[&frame.vertex] {
            let mut scc = HashSet::new();
            loop {
                let w = stack.pop().unwrap();
                on_stack.insert(w, false);
                scc.insert(w);
                if w == frame.vertex {
                    break;
                }
            }
            sccs.push(scc);
        }
    }
}

pub fn tarjan_scc(graph: &Graph) -> Vec<HashSet<u32>> {
    let mut index_counter = 0;
    let mut indexes = HashMap::new();
    let mut lowlinks = HashMap::new();
    let mut stack = Vec::new();
    let mut on_stack = HashMap::new();
    let mut sccs = Vec::new();

    for v in graph.vertices() {
        if !indexes.contains_key(&v) {
            strong_connect(
                v,
                graph,
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
pub fn get_number_of_comps(comps: &Vec<HashSet<u32>>) -> u32 {
    comps.len() as u32
}

#[allow(clippy::ptr_arg)]
pub fn fraction_in_largest_component(comp: &HashSet<u32>, num_vertices: usize) -> f64 {
    comp.len() as f64 / num_vertices as f64
}

pub fn get_largest_comp(comps: &Vec<HashSet<u32>>) -> HashSet<u32> {
    let mut largest: HashSet<u32> = HashSet::new();
    comps.iter().map(|comp| if comp.len() > largest.len() {largest = comp.clone()});
    largest
}