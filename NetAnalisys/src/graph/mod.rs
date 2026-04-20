use super::parser::directed_or_undirected::DirectedOrUndirected;
use std::collections::HashMap;
pub mod traversal;

pub struct Graph {
    pub adjacency_list: HashMap<u32, Vec<u32>>,
    pub graph_type: DirectedOrUndirected,
}

impl Graph {
    pub fn new(graph_type: DirectedOrUndirected) -> Self {
        Self {
            adjacency_list: HashMap::new(),
            graph_type,
        }
    }

    pub fn num_vertices(&self) -> usize {
        self.adjacency_list.len()
    }

    pub fn num_edges(&self) -> usize {
        let total: usize = self.adjacency_list.values().map(|v| v.len()).sum();
        match self.graph_type {
            DirectedOrUndirected::Undirected => total / 2,
            DirectedOrUndirected::Directed => total,
        }
    }
}
