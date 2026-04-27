use std::error::Error;

use super::directed_or_undirected::DirectedOrUndirected;
use crate::graph::Graph;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

type DynError = Box<dyn Error + Send + Sync>;

#[allow(dead_code)]
pub async fn txt_parser(path: &str, graph_type: &DirectedOrUndirected) -> Result<Graph, DynError> {
    let file = File::open(path).await?;
    let reader = BufReader::new(file);
    let mut graph = Graph::new(*graph_type);
    let mut lines = reader.lines();
    while let Some(line) = lines.next_line().await? {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }
        if line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();

        let u: u32 = parts[0].parse()?;
        let v: u32 = parts[1].parse()?;

        graph.add_edge(u, v);
    }
    Ok(graph)
}
