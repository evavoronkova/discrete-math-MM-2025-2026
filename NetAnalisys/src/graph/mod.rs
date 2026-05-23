use super::parser::directed_or_undirected::DirectedOrUndirected;
use rustc_hash::FxHashMap as HashMap;

pub mod traversal;

#[derive(Debug)]
pub struct Graph {
    adjacency_list: Vec<Vec<u32>>,
    id_map: HashMap<u32, u32>,
    reverse_id_map: Vec<u32>,
    graph_type: DirectedOrUndirected,
    num_edges: usize,
    out_degree: Vec<u32>,
    in_degree: Vec<u32>,
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
            id_map: HashMap::default(),
            reverse_id_map: Vec::new(),
            graph_type,
            num_edges: 0,
            out_degree: Vec::new(),
            in_degree: Vec::new(),
        }
    }

    /// Normalize adjacency lists and precompute aggregates.
    /// Must be called after all edges have been added.
    pub fn finalize(&mut self) {
        let n = self.adjacency_list.len();
        self.out_degree = vec![0u32; n];
        self.in_degree = vec![0u32; n];

        for (v, neighbors) in self.adjacency_list.iter_mut().enumerate() {
            // Sort and remove duplicate edges
            neighbors.sort_unstable();
            neighbors.dedup();
            // Remove self-loops
            neighbors.retain(|&u| u != v as u32);
            self.out_degree[v] = neighbors.len() as u32;
            for &u in neighbors.iter() {
                self.in_degree[u as usize] += 1;
            }
        }

        match self.graph_type {
            DirectedOrUndirected::Undirected => {
                self.num_edges = self.out_degree.iter().map(|&d| d as usize).sum::<usize>() / 2;
                // For undirected, in_degree == out_degree
                self.in_degree = self.out_degree.clone();
            }
            DirectedOrUndirected::Directed => {
                self.num_edges = self.out_degree.iter().map(|&d| d as usize).sum::<usize>();
            }
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

    /// Out-degree by internal vertex id.
    pub fn out_degree(&self, vertex: u32) -> u32 {
        self.out_degree.get(vertex as usize).copied().unwrap_or(0)
    }

    /// In-degree by internal vertex id (for directed graphs; equals out-degree for undirected).
    pub fn in_degree(&self, vertex: u32) -> u32 {
        self.in_degree.get(vertex as usize).copied().unwrap_or(0)
    }

    /// All out-degrees indexed by internal vertex id.
    pub fn out_degrees(&self) -> &[u32] {
        &self.out_degree
    }

    /// All in-degrees indexed by internal vertex id.
    pub fn in_degrees(&self) -> &[u32] {
        &self.in_degree
    }

    pub fn add_vertex(&mut self, vertex: u32) {
        self.get_or_create_id(vertex);
    }

    pub fn add_edge(&mut self, source: u32, target: u32) {
        let source_id = self.get_or_create_id(source);
        let target_id = self.get_or_create_id(target);

        self.adjacency_list[source_id as usize].push(target_id);
        if matches!(self.graph_type, DirectedOrUndirected::Undirected) {
            self.adjacency_list[target_id as usize].push(source_id);
        }
    }

    /// Add an edge using already-known internal IDs.
    /// The caller must ensure both vertices exist.
    pub fn add_edge_internal(&mut self, source: u32, target: u32) {
        self.adjacency_list[source as usize].push(target);
        if matches!(self.graph_type, DirectedOrUndirected::Undirected) {
            self.adjacency_list[target as usize].push(source);
        }
    }

    /// Copy vertex mapping from another graph (same internal ids assumed).
    pub fn init_mapping_from(&mut self, other: &Graph) {
        self.id_map = other.id_map.clone();
        self.reverse_id_map = other.reverse_id_map.clone();
        self.adjacency_list = vec![Vec::new(); other.num_vertices()];
    }

    /// Returns neighbors as external IDs (allocates a Vec on each call).
    pub fn neighbors(&self, vertex: u32) -> Vec<u32> {
        self.get_internal_id(vertex)
            .map(|id| {
                self.adjacency_list
                    .get(id as usize)
                    .map(|neighbors| {
                        neighbors
                            .iter()
                            .map(|&v| self.reverse_id_map[v as usize])
                            .collect()
                    })
                    .unwrap_or_default()
            })
            .unwrap_or_default()
    }

    pub fn vertices(&self) -> impl Iterator<Item = u32> + '_ {
        self.reverse_id_map.iter().copied()
    }

    /// External-ID adjacency iterator (allocates a Vec per vertex).
    pub fn adjacency_entries(&self) -> impl Iterator<Item = (u32, Vec<u32>)> + '_ {
        self.adjacency_list
            .iter()
            .enumerate()
            .map(|(vertex, neighbors)| {
                let ext_neighbors: Vec<u32> = neighbors
                    .iter()
                    .map(|&v| self.reverse_id_map[v as usize])
                    .collect();
                (self.reverse_id_map[vertex], ext_neighbors)
            })
    }

    pub fn has_edge(&self, source: u32, target: u32) -> bool {
        match (self.get_internal_id(source), self.get_internal_id(target)) {
            (Some(source_id), Some(target_id)) => self.adjacency_list[source_id as usize]
                .binary_search(&target_id)
                .is_ok(),
            _ => false,
        }
    }

    pub fn has_edge_internal(&self, source: u32, target: u32) -> bool {
        self.adjacency_list
            .get(source as usize)
            .map(|neighbors| neighbors.binary_search(&target).is_ok())
            .unwrap_or(false)
    }

    pub fn num_vertices(&self) -> usize {
        self.reverse_id_map.len()
    }

    pub fn num_edges(&self) -> usize {
        self.num_edges
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
