pub struct Graph {
    pub adjacency_list: std::collections::HashMap<u32, Vec<u32>>,
    pub graph_type: crate::parser::directed_or_undirected::DirectedOrUndirected,
} 