use super::directed_or_undirected::DirectedOrUndirected;
use super::file_type::FileType;
use crate::graph::Graph;
use crate::parser::{parse_from_csv, parse_from_mtx, parse_from_txt};
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use tokio;

#[allow(dead_code)]
fn parse_type_file(path: &str) -> Result<(FileType, DirectedOrUndirected), Box<dyn Error>> {
    let path = path.trim();

    if path.is_empty() {
        return Err("Path to file can't be empty".into());
    }

    let path = Path::new(path);
    let direction = path
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|name| name.to_str())
        .map(|folder| folder.to_lowercase());

    let graph_type = match direction.as_deref() {
        Some("directed") => DirectedOrUndirected::Directed,
        Some("undirected") => DirectedOrUndirected::Undirected,
        Some(dir) => {
            return Err(format!(
                "Expected a 'directed' or 'undirected' folder, found '{}'",
                dir
            )
            .into());
        }
        None => {
            return Err("The file must be inside the 'directed' or 'undirected' folder.".into());
        }
    };

    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase());
    match extension.as_deref() {
        Some("txt") => Ok((FileType::Txt, graph_type)),
        Some("mtx") => Ok((FileType::Mtx, graph_type)),
        Some("csv") => Ok((FileType::Csv, graph_type)),
        Some(ext) => Err(format!("Extension '{}' is not supported", ext).into()),
        None => Err("File has no extension".into()),
    }
}

pub async fn parse_file(path: &str) -> Result<Graph, Box<dyn Error>> {
    let (file_type, graph_type) = parse_type_file(path)?;

    let adjacency_list = match file_type {
        FileType::Txt => parse_from_txt::txt_parser(path, &graph_type).await,
        FileType::Csv => parse_from_csv::csv_parser(path, &graph_type).await,
        FileType::Mtx => parse_from_mtx::mtx_parser(path, &graph_type).await,
    };

    match adjacency_list {
        Ok(adjacency_list) => Ok(Graph {
            adjacency_list,
            graph_type,
        }),
        Err(e) => Err(e),
    }
}
