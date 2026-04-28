use std::error::Error;

use super::directed_or_undirected::DirectedOrUndirected;
use crate::graph::Graph;
use rayon::prelude::*;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::task;

type DynError = Box<dyn Error + Send + Sync>;

pub async fn mtx_parser(path: &str, graph_type: &DirectedOrUndirected) -> Result<Graph, DynError> {
    let file = File::open(path).await?;
    let reader = BufReader::with_capacity(1 << 20, file);

    let mut lines = reader.lines();
    let mut graph = Graph::new(*graph_type);
    let mut skipped_shape_line = false;

    const CHUNK_SIZE: usize = 50_000;
    let mut buffer = Vec::with_capacity(CHUNK_SIZE);

    while let Some(line) = lines.next_line().await? {
        let line = line.trim();

        if line.is_empty() || line.starts_with('%') {
            continue;
        }

        if !skipped_shape_line {
            skipped_shape_line = true;
            continue;
        }

        buffer.push(line.to_string());

        if buffer.len() >= CHUNK_SIZE {
            process_chunk(&mut graph, std::mem::take(&mut buffer)).await?;
        }
    }

    if !buffer.is_empty() {
        process_chunk(&mut graph, buffer).await?;
    }

    Ok(graph)
}

async fn process_chunk(graph: &mut Graph, chunk: Vec<String>) -> Result<(), DynError> {
    let edges = task::spawn_blocking(move || {
        chunk
            .par_iter()
            .filter_map(|line| parse_edge_fast(line))
            .collect::<Vec<(u32, u32)>>()
    })
    .await?;

    for (u, v) in edges {
        graph.add_edge(u, v);
    }

    Ok(())
}

#[inline(always)]
fn parse_edge_fast(line: &str) -> Option<(u32, u32)> {
    let mut parts = line.split_ascii_whitespace();
    let u = parts.next()?.parse().ok()?;
    let v = parts.next()?.parse().ok()?;
    Some((u, v))
}
