use super::parser::directed_or_undirected::DirectedOrUndirected;
use rustc_hash::FxHashMap as HashMap;
pub mod traversal;

#[derive(Debug)]
pub struct Graph {
    adjacency_list: HashMap<u32, Vec<u32>>,
    graph_type: DirectedOrUndirected,
}

impl Graph {
    pub fn new(graph_type: DirectedOrUndirected) -> Self {
        Self {
            adjacency_list: HashMap::default(),
            graph_type,
        }
    }

    pub fn kind(&self) -> DirectedOrUndirected {
        self.graph_type
    }

    pub fn add_vertex(&mut self, vertex: u32) {
        self.adjacency_list.entry(vertex).or_default();
    }

    pub fn add_edge(&mut self, source: u32, target: u32) {
        self.adjacency_list.entry(source).or_default().push(target);
        self.add_vertex(target);

        if matches!(self.graph_type, DirectedOrUndirected::Undirected) {
            self.adjacency_list.entry(target).or_default().push(source);
        }
    }

    pub fn neighbors(&self, vertex: u32) -> &[u32] {
        self.adjacency_list
            .get(&vertex)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub fn vertices(&self) -> impl Iterator<Item = u32> + '_ {
        self.adjacency_list.keys().copied()
    }

    pub fn adjacency_entries(&self) -> impl Iterator<Item = (u32, &[u32])> + '_ {
        self.adjacency_list
            .iter()
            .map(|(&vertex, neighbors)| (vertex, neighbors.as_slice()))
    }

    pub fn has_edge(&self, source: u32, target: u32) -> bool {
        self.neighbors(source).contains(&target)
    }

    pub fn num_vertices(&self) -> usize {
        self.adjacency_list.len()
    }

    pub fn num_edges(&self) -> usize {
        let total: usize = self.adjacency_list.values().map(|v| v.len()).sum();
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
