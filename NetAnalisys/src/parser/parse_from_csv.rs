use super::directed_or_undirected::DirectedOrUndirected;
use std::collections::HashMap;
use std::error::Error;

fn csv_parser(
    path: &str,
    graph_type: DirectedOrUndirected,
) -> Result<HashMap<u32, Vec<u32>>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut adjacency_list: HashMap<u32, Vec<u32>> = HashMap::new();

    for result in reader.records() {
        let record = result?;

        let node: u32 = record.get(0).ok_or("Missing node")?.parse()?;
        let neighbour: u32 = record.get(1).ok_or("Missing neighbour")?.parse()?;

        adjacency_list.entry(node).or_default().push(neighbour);

        if let DirectedOrUndirected::Undirected = graph_type {
            adjacency_list.entry(neighbour).or_default().push(node);
        }
    }

    Ok(adjacency_list)
}
