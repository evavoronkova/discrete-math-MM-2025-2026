mod parser;
mod graph;


#[allow(dead_code)]
fn calculate_main_graph_information(graph: &graph::Graph) {
    let num_vertices = graph.adjacency_list.len();
    let mut num_edges: usize = graph
        .adjacency_list
        .values()
        .map(|neighbors| neighbors.len())
        .sum();

    if let parser::directed_or_undirected::DirectedOrUndirected::Undirected = graph.graph_type {
        num_edges /= 2;
    }
}
fn main() {


}
