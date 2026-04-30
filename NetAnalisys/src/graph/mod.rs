use super::parser::directed_or_undirected::DirectedOrUndirected;
use rayon::prelude::*;
use rustc_hash::FxHashMap as HashMap;
pub mod traversal;

#[derive(Debug)]
pub struct Graph {
    adjacency_list: Vec<Vec<u32>>,

    adjacency_external: Vec<Vec<u32>>,
    id_map: HashMap<u32, u32>,
    reverse_id_map: Vec<u32>,
    graph_type: DirectedOrUndirected,
}

#[allow(unused)]
impl Graph {
    fn get_or_create_id(&mut self, vertex: u32) -> u32 {
        if let Some(&id) = self.id_map.get(&vertex) {
            return id;
        }

        let new_id = self.adjacency_list.len() as u32;

        self.id_map.insert(vertex, new_id);
        self.adjacency_list.push(Vec::new());
        self.adjacency_external.push(Vec::new());
        self.reverse_id_map.push(vertex);

        new_id
    }

    fn get_internal_id(&self, vertex: u32) -> Option<u32> {
        self.id_map.get(&vertex).copied()
    }

    fn get_external_id(&self, vertex: u32) -> Option<u32> {
        self.reverse_id_map.get(vertex as usize).copied()
    }

    pub fn new(graph_type: DirectedOrUndirected) -> Self {
        Self {
            adjacency_list: Vec::new(),
            adjacency_external: Vec::new(),
            id_map: HashMap::default(),
            reverse_id_map: Vec::new(),
            graph_type,
        }
    }

    pub fn kind(&self) -> DirectedOrUndirected {
        self.graph_type
    }

    pub fn external_to_internal(&self, vertex: u32) -> Option<u32> {
        self.get_internal_id(vertex)
    }

    pub fn internal_to_external(&self, vertex: u32) -> Option<u32> {
        self.get_external_id(vertex)
    }

    pub fn neighbors_internal(&self, vertex: u32) -> &[u32] {
        self.adjacency_list
            .get(vertex as usize)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub fn vertices_internal(&self) -> impl Iterator<Item = u32> + '_ {
        0..self.adjacency_list.len() as u32
    }

    pub fn adjacency_entries_internal(&self) -> impl Iterator<Item = (u32, &[u32])> + '_ {
        self.adjacency_list
            .iter()
            .enumerate()
            .map(|(vertex, neighbors)| (vertex as u32, neighbors.as_slice()))
    }

    pub fn add_vertex(&mut self, vertex: u32) {
        self.get_or_create_id(vertex);
    }

    pub fn add_edge(&mut self, source: u32, target: u32) {
        let source_id = self.get_or_create_id(source);
        let target_id = self.get_or_create_id(target);

        self.adjacency_list[source_id as usize].push(target_id);
        self.adjacency_external[source_id as usize].push(target);
        if matches!(self.graph_type, DirectedOrUndirected::Undirected) {
            self.adjacency_list[target_id as usize].push(source_id);
            self.adjacency_external[target_id as usize].push(source);
        }
    }

    pub fn neighbors(&self, vertex: u32) -> &[u32] {
        self.get_internal_id(vertex)
            .and_then(|id| self.adjacency_external.get(id as usize))
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub fn vertices(&self) -> impl Iterator<Item = u32> + '_ {
        self.reverse_id_map.iter().copied()
    }

    pub fn adjacency_entries(&self) -> impl Iterator<Item = (u32, &[u32])> + '_ {
        self.adjacency_external
            .iter()
            .enumerate()
            .map(|(vertex, neighbors)| (self.reverse_id_map[vertex], neighbors.as_slice()))
    }

    pub fn has_edge(&self, source: u32, target: u32) -> bool {
        match (self.get_internal_id(source), self.get_internal_id(target)) {
            (Some(source_id), Some(target_id)) => {
                self.adjacency_list[source_id as usize].contains(&target_id)
            }
            _ => false,
        }
    }

    pub fn has_edge_internal(&self, source: u32, target: u32) -> bool {
        self.adjacency_list
            .get(source as usize)
            .map(|neighbors| neighbors.contains(&target))
            .unwrap_or(false)
    }

    pub fn num_vertices(&self) -> usize {
        self.reverse_id_map.len()
    }

    pub fn num_edges(&self) -> usize {
        let total: usize = self.adjacency_list.par_iter().map(|v| v.len()).sum();
        match self.kind() {
            DirectedOrUndirected::Undirected => total / 2,
            DirectedOrUndirected::Directed => total,
        }
    }

    pub fn density(&self, num_vertices: usize, num_edges: usize) -> f64 {
        if num_vertices < 2 {
            return 0.0;
        }

        match self.kind() {
            DirectedOrUndirected::Undirected => {
                (2.0 * num_edges as f64) / ((num_vertices * (num_vertices - 1)) as f64)
            }
            DirectedOrUndirected::Directed => {
                (num_edges as f64) / ((num_vertices * (num_vertices - 1)) as f64)
            }
        }
    }
}
