use super::directed_or_undirected::DirectedOrUndirected;
use std::collections::HashMap;
use std::error::Error;

#[allow(dead_code)]
pub fn csv_parser(
    path: &str,
    graph_type: DirectedOrUndirected,
) -> Result<HashMap<u32, Vec<u32>>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut adjacency_list: HashMap<u32, Vec<u32>> = HashMap::new();

    for (i, result) in reader.records().enumerate() {
        let record = result?;

        if record.is_empty() {
            continue;
        }

        if record.len() != 2 {
            eprintln!(
                "Skipped line {}: expected 2 columns, got {}",
                i + 1,
                record.len()
            );
            continue;
        }

        let node: u32 = record
            .get(0)
            .ok_or("Missing node")?
            .parse()
            .map_err(|e| format!("Line {}: invalid node: {}", i + 1, e))?;

        let neighbour: u32 = record
            .get(1)
            .ok_or("Missing neighbour")?
            .parse()
            .map_err(|e| format!("Line {}: invalid neighbour: {}", i + 1, e))?;

        adjacency_list.entry(node).or_default().push(neighbour);
        adjacency_list.entry(neighbour).or_default();

        if let DirectedOrUndirected::Undirected = graph_type {
            adjacency_list.entry(neighbour).or_default().push(node);
        }
    }

    Ok(adjacency_list)
}
