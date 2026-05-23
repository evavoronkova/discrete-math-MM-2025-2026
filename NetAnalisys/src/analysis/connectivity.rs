use crate::graph::Graph;
use crate::parser::directed_or_undirected::DirectedOrUndirected;
use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;
use rustc_hash::FxHashSet as HashSet;

/// Build an undirected version of the graph using internal IDs directly.
pub fn build_undirected(graph: &Graph) -> Graph {
    let mut undirected_graph = Graph::new(DirectedOrUndirected::Undirected);
    undirected_graph.init_mapping_from(graph);

    for (u, neighbors) in graph.adjacency_entries_internal() {
        for &v in neighbors {
            undirected_graph.add_edge_internal(u, v);
        }
    }

    undirected_graph.finalize();
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
    let mut visited = vec![false; graph_ref.num_vertices()];
    let mut components = Vec::new();

    for vertex in graph_ref.vertices_internal() {
        if !visited[vertex as usize] {
            let mut comp = HashSet::default();
            let mut stack = vec![vertex];

            while let Some(node) = stack.pop() {
                if visited[node as usize] {
                    continue;
                }

                visited[node as usize] = true;
                comp.insert(graph_ref.internal_to_external(node).unwrap());

                for &neighbor in graph_ref.neighbors_internal(node) {
                    if !visited[neighbor as usize] {
                        stack.push(neighbor);
                    }
                }
            }

            components.push(comp);
        }
    }

    components
}

pub fn tarjan_scc(graph: &Graph) -> Vec<HashSet<u32>> {
    let mut index_counter: u32 = 0;
    let mut indexes = vec![0_u32; graph.num_vertices()];
    let mut lowlinks = vec![0_u32; graph.num_vertices()];
    let mut stack = Vec::new();
    let mut on_stack = vec![false; graph.num_vertices()];
    let mut sccs = Vec::new();

    struct Frame {
        vertex: u32,
        next_neighbor_idx: usize,
        parent: Option<u32>,
    }

    for start in graph.vertices_internal() {
        if indexes[start as usize] != 0 {
            continue;
        }

        index_counter += 1;
        indexes[start as usize] = index_counter;
        lowlinks[start as usize] = index_counter;
        stack.push(start);
        on_stack[start as usize] = true;

        let mut frames = vec![Frame {
            vertex: start,
            next_neighbor_idx: 0,
            parent: None,
        }];

        while let Some(mut frame) = frames.pop() {
            let neighbors = graph.neighbors_internal(frame.vertex);

            if frame.next_neighbor_idx < neighbors.len() {
                let current_vertex = frame.vertex;
                let neighbor = neighbors[frame.next_neighbor_idx];
                frame.next_neighbor_idx += 1;
                frames.push(frame);

                if indexes[neighbor as usize] == 0 {
                    index_counter += 1;
                    indexes[neighbor as usize] = index_counter;
                    lowlinks[neighbor as usize] = index_counter;
                    stack.push(neighbor);
                    on_stack[neighbor as usize] = true;

                    frames.push(Frame {
                        vertex: neighbor,
                        next_neighbor_idx: 0,
                        parent: Some(current_vertex),
                    });
                } else if on_stack[neighbor as usize] {
                    let new_lowlink =
                        lowlinks[current_vertex as usize].min(indexes[neighbor as usize]);
                    lowlinks[current_vertex as usize] = new_lowlink;
                }

                continue;
            }

            if let Some(parent) = frame.parent {
                let parent_lowlink = lowlinks[parent as usize].min(lowlinks[frame.vertex as usize]);
                lowlinks[parent as usize] = parent_lowlink;
            }

            if lowlinks[frame.vertex as usize] == indexes[frame.vertex as usize] {
                let mut scc = HashSet::default();
                loop {
                    let w = stack.pop().unwrap();
                    on_stack[w as usize] = false;
                    scc.insert(graph.internal_to_external(w).unwrap());
                    if w == frame.vertex {
                        break;
                    }
                }
                sccs.push(scc);
            }
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

pub fn fraction_from_component_size(component_size: usize, num_vertices: usize) -> f64 {
    component_size as f64 / num_vertices as f64
}

pub fn largest_component_size(comps: &[HashSet<u32>]) -> usize {
    comps.iter().map(HashSet::len).max().unwrap_or(0)
}

pub fn get_largest_comp(comps: &[HashSet<u32>]) -> HashSet<u32> {
    comps
        .iter()
        .max_by_key(|comp| comp.len())
        .cloned()
        .unwrap_or_default()
}

pub fn find_weak_components_masked(
    graph: &Graph,
    allowed_mask: Option<&[bool]>,
    num_vertices: usize,
) -> Vec<HashSet<u32>> {
    let owned_graph: Graph;
    let graph_ref = match graph.kind() {
        DirectedOrUndirected::Undirected => graph,
        DirectedOrUndirected::Directed => {
            owned_graph = build_undirected(graph);
            &owned_graph
        }
    };

    let mut visited = vec![false; num_vertices];
    let mut components = Vec::new();

    for vertex in graph_ref.vertices_internal() {
        if let Some(mask) = allowed_mask
            && !mask[vertex as usize]
        {
            continue;
        }
        if visited[vertex as usize] {
            continue;
        }

        let mut comp = HashSet::default();
        let mut stack = vec![vertex];

        while let Some(node) = stack.pop() {
            if visited[node as usize] {
                continue;
            }
            if let Some(mask) = allowed_mask
                && !mask[node as usize]
            {
                continue;
            }

            visited[node as usize] = true;
            comp.insert(graph_ref.internal_to_external(node).unwrap());

            for &neighbor in graph_ref.neighbors_internal(node) {
                if !visited[neighbor as usize] {
                    if let Some(mask) = allowed_mask
                        && !mask[neighbor as usize]
                    {
                        continue;
                    }
                    stack.push(neighbor);
                }
            }
        }

        if !comp.is_empty() {
            components.push(comp);
        }
    }
    components
}
