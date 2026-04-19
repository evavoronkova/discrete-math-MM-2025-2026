use crate::graph::Graph;

#[allow(dead_code)]
fn calculate_mid_k(graph: Graph) -> f64 {
    let mut local_k = Vec::new();
    for (node, neighbors) in &graph.adjacency_list {
        let neighbor_count = neighbors.len();
        let mut actual_edges = 0;
        let max_edges = neighbor_count * (neighbor_count - 1) / 2;
        if max_edges == 0 {
            local_k.push(0.0);
            continue;
        }
        for first_neighbor in neighbors.iter() {
            for second_neighbor in neighbors.iter() {
                if first_neighbor < second_neighbor
                    && graph.adjacency_list[first_neighbor].contains(second_neighbor)
                {
                    actual_edges += 1;
                }
            }
        }
        local_k.push(actual_edges as f64 / max_edges as f64);
    }
    local_k.iter().sum::<f64>() / graph.adjacency_list.len() as f64
}

#[allow(dead_code)]
fn triplet_counter(graph: &Graph) -> u32 {
    let mut triplets_count = 0;
    for (node, neighbors) in &graph.adjacency_list {
        let node_degree = neighbors.len();
        triplets_count += (node_degree * (node_degree - 1) / 2) as u32;
    }
    triplets_count
}