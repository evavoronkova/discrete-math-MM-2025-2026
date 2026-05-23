use super::directed_or_undirected::DirectedOrUndirected;
use super::file_type::FileType;
use crate::graph::Graph;
use rayon::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

type DynError = Box<dyn Error + Send + Sync>;

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
        _ => DirectedOrUndirected::Undirected,
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

pub fn parse_file(path: &str) -> Result<Graph, Box<dyn Error>> {
    let (file_type, graph_type) = parse_type_file(path)?;

    let graph = match file_type {
        FileType::Txt => parse_from_sample(path, &graph_type, '#', ' ', false),
        FileType::Csv => parse_from_sample(path, &graph_type, '#', ',', false),
        FileType::Mtx => parse_from_sample(path, &graph_type, '%', ' ', true),
    };

    graph.map_err(|e| e as Box<dyn Error>)
}

fn parse_from_sample(
    path: &str,
    graph_type: &DirectedOrUndirected,
    symbol_skip: char,
    symbol_chunks: char,
    skip_first_line: bool,
) -> Result<Graph, DynError> {
    let file = File::open(path)?;
    let reader = BufReader::with_capacity(1 << 20, file);

    let mut graph = Graph::new(*graph_type);
    const CHUNK_SIZE: usize = 50_000;
    let mut buffer = Vec::with_capacity(CHUNK_SIZE);
    let mut skipped_shape_line: bool = !skip_first_line;

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() || line.starts_with(symbol_skip) {
            continue;
        }

        if !skipped_shape_line {
            skipped_shape_line = true;
            continue;
        }

        buffer.push(line.to_string());
        if buffer.len() >= CHUNK_SIZE {
            process_chunk(&mut graph, std::mem::take(&mut buffer), symbol_chunks)?;
        }
    }
    if !buffer.is_empty() {
        process_chunk(&mut graph, buffer, symbol_chunks)?;
    }

    graph.finalize();

    Ok(graph)
}

fn process_chunk(graph: &mut Graph, chunk: Vec<String>, symbol: char) -> Result<(), DynError> {
    let edges: Vec<(u32, u32)> = chunk
        .par_iter()
        .filter_map(|line| parse_edge_fast(line, symbol))
        .collect();

    for (u, v) in edges {
        graph.add_edge(u, v);
    }

    Ok(())
}

#[inline(always)]
fn parse_edge_fast(line: &str, symbol: char) -> Option<(u32, u32)> {
    if symbol == ' ' {
        let mut parts = line.split_ascii_whitespace();
        let u = parts.next()?.parse().ok()?;
        let v = parts.next()?.parse().ok()?;
        Some((u, v))
    } else {
        let mut parts = line.split(symbol);
        let u = parts.next()?.trim().parse().ok()?;
        let v = parts.next()?.trim().parse().ok()?;
        Some((u, v))
    }
}
