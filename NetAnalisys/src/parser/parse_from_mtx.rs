use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;

use super::directed_or_undirected::DirectedOrUndirected;

pub fn mtx_parser(
    path: &str,
    graph_type: &DirectedOrUndirected,
) -> Result<HashMap<u32, Vec<u32>>, Box<dyn Error>> {
    let content = read_to_string(path)?;
    let mut adjacency_list: HashMap<u32, Vec<u32>> = HashMap::new();

    for line in content.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with('%') {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() != 2 {
            continue;
        }

        let u: u32 = parts[0].parse()?;
        let v: u32 = parts[1].parse()?;

        adjacency_list.entry(u).or_default().push(v);
        adjacency_list.entry(v).or_default();

        if let DirectedOrUndirected::Undirected = graph_type {
            adjacency_list.entry(v).or_default().push(u);
        }
    }

    Ok(adjacency_list)
}
