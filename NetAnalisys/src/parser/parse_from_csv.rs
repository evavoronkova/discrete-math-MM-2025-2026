use std::error::Error;

use super::directed_or_undirected::DirectedOrUndirected;
use crate::graph::Graph;

use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

type DynError = Box<dyn Error + Send + Sync>;

use rayon::prelude::*;
use std::fs;

pub async fn csv_parser(path: &str, graph_type: &DirectedOrUndirected) -> Result<Graph, DynError> {
    let path = path.to_string();
    let graph_type = graph_type.clone();

    let graph = tokio::task::spawn_blocking(move || {
        let content = fs::read_to_string(path)?;

        let edges: Vec<(u32, u32)> = content
            .par_lines()
            .filter_map(|line| {
                let mut parts = line.split(',');

                let u = parts.next()?.trim().parse().ok()?;
                let v = parts.next()?.trim().parse().ok()?;

                Some((u, v))
            })
            .collect();

        let mut graph = Graph::new(graph_type);

        for (u, v) in edges {
            graph.add_edge(u, v);
        }

        Ok::<_, DynError>(graph)
    })
    .await??;

    Ok(graph)
}
