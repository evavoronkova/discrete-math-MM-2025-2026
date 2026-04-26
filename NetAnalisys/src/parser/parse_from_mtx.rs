use std::collections::HashMap;
use std::error::Error;

use tokio::fs::File;
use tokio::io::{self, AsyncBufReadExt, BufReader};

use super::directed_or_undirected::DirectedOrUndirected;

type DynError = Box<dyn Error + Send + Sync>;

pub async fn mtx_parser(
    path: &str,
    graph_type: &DirectedOrUndirected,
) -> Result<HashMap<u32, Vec<u32>>, DynError> {
    let file = File::open(path).await?;
    let reader = BufReader::new(file);
    let mut adjacency_list: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut lines = reader.lines();
    while let Some(line) = lines.next_line().await? {
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
