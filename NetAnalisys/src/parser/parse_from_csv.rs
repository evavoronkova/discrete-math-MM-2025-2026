use crate::graph;
use super::directed_or_undirected::DirectedOrUndirected;
use std::collections::HashMap;
use std::error::Error;
use tokio::task;

type DynError = Box<dyn Error + Send + Sync>;

pub async fn csv_parser(
    path: &str,
    graph_type: &DirectedOrUndirected,
) -> Result<HashMap<u32, Vec<u32>>, DynError> {
    let path = path.to_string();
    let graph_type = graph_type.clone();

    let adjacency_list = task::spawn_blocking(move || {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(path)?;

        let mut adjacency_list: HashMap<u32, Vec<u32>> = HashMap::new();

        for result in reader.records() {
            let record = result?;

            if record.len() != 2 {
                continue;
            }

            let u: u32 = record[0].parse()?;
            let v: u32 = record[1].parse()?;

            adjacency_list.entry(u).or_default().push(v);
            adjacency_list.entry(v).or_default();

            if let DirectedOrUndirected::Undirected = graph_type {
                adjacency_list.entry(v).or_default().push(u);
            }
        }

        Ok::<_, DynError>(adjacency_list)
    })
    .await??;

    Ok(adjacency_list)
}
