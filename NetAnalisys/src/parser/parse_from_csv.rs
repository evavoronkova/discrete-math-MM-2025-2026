use super::directed_or_undirected::DirectedOrUndirected;
use crate::graph::Graph;
use std::error::Error;
use tokio::task;

type DynError = Box<dyn Error + Send + Sync>;

pub async fn csv_parser(
    path: &str,
    graph_type: &DirectedOrUndirected,
) -> Result<Graph, DynError> {
    let path = path.to_string();
    let graph_type = graph_type.clone();

    let graph = task::spawn_blocking(move || {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(path)?;

        let mut graph = Graph::new(graph_type);

        for result in reader.records() {
            let record = result?;

            if record.len() != 2 {
                continue;
            }

            let u: u32 = record[0].parse()?;
            let v: u32 = record[1].parse()?;

            graph.add_edge(u, v);
        }

        Ok::<_, DynError>(graph)
    })
    .await??;

    Ok(graph)
}
