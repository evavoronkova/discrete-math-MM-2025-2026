use std::error::Error;

use crate::graph::Graph;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

use super::directed_or_undirected::DirectedOrUndirected;

type DynError = Box<dyn Error + Send + Sync>;

pub async fn mtx_parser(path: &str, graph_type: &DirectedOrUndirected) -> Result<Graph, DynError> {
    let file = File::open(path).await?;
    let reader = BufReader::new(file);
    let mut graph = Graph::new(*graph_type);
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

        graph.add_edge(u, v);
    }

    Ok(graph)
}
