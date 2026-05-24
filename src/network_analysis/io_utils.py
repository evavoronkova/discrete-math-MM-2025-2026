import csv
from pathlib import Path
from typing import Iterable, Iterator

from .graph import Graph


def _iter_plain_edges(path: Path, delimiter: str | None = None) -> Iterator[tuple[str, str]]:
    with path.open("r", encoding="utf-8", errors="replace") as handle:
        for raw_line in handle:
            line = raw_line.strip()
            if not line or line.startswith("#") or line.startswith("%"):
                continue
            parts = line.split(delimiter) if delimiter else line.split()
            if len(parts) < 2:
                continue
            yield parts[0], parts[1]


def _iter_size_header_edges(path: Path, delimiter: str | None = None) -> Iterator[tuple[str, str]]:
    with path.open("r", encoding="utf-8", errors="replace") as handle:
        header_skipped = False
        for raw_line in handle:
            line = raw_line.strip()
            if not line or line.startswith("#") or line.startswith("%"):
                continue
            if not header_skipped:
                header_skipped = True
                continue
            parts = line.split(delimiter) if delimiter else line.split()
            if len(parts) < 2:
                continue
            yield parts[0], parts[1]


def _iter_csv_edges(path: Path) -> Iterator[tuple[str, str]]:
    with path.open("r", encoding="utf-8", errors="replace", newline="") as handle:
        reader = csv.reader(handle)
        header_checked = False
        for row in reader:
            if not row or len(row) < 2:
                continue
            if not header_checked:
                header_checked = True
                if not _looks_like_edge(row[0], row[1]):
                    continue
            yield row[0].strip(), row[1].strip()


def _iter_mtx_edges(path: Path) -> Iterator[tuple[str, str]]:
    with path.open("r", encoding="utf-8", errors="replace") as handle:
        size_line_skipped = False
        for raw_line in handle:
            line = raw_line.strip()
            if not line or line.startswith("%"):
                continue
            if not size_line_skipped:
                size_line_skipped = True
                continue
            parts = line.split()
            if len(parts) < 2:
                continue
            yield parts[0], parts[1]


def _looks_like_edge(left: str, right: str) -> bool:
    if not left or not right:
        return False
    lowered = (left.lower(), right.lower())
    forbidden = {"id_1", "id_2", "source", "target", "fromnodeid", "tonodeid", "u", "v"}
    if lowered[0] in forbidden or lowered[1] in forbidden:
        return False
    return True


def detect_directed_from_name(path: str | Path) -> bool:
    file_path = Path(path)
    lowered_parts = tuple(part.lower() for part in file_path.parts)
    if "datasets" in lowered_parts and "undirected" in lowered_parts:
        return False
    name = file_path.name.lower()
    directed_markers = ("email", "wiki-vote", "vote", "web-")
    undirected_markers = ("ca-", "musae", "grqc", "astroph", "youtube", "dblp", "vk")
    if any(marker in name for marker in directed_markers):
        return True
    if any(marker in name for marker in undirected_markers):
        return False
    return False


def detect_format(path: str | Path) -> str:
    suffix = Path(path).suffix.lower()
    if suffix == ".csv":
        return "csv"
    if suffix == ".mtx":
        return "mtx"
    return "edge_list"


def iter_edges(path: str | Path, file_format: str = "auto", delimiter: str | None = None) -> Iterable[tuple[str, str]]:
    file_path = Path(path)
    resolved_format = detect_format(file_path) if file_format == "auto" else file_format
    if resolved_format == "csv":
        return _iter_csv_edges(file_path)
    if resolved_format == "mtx":
        return _iter_mtx_edges(file_path)
    if resolved_format == "size_header_edge_list":
        return _iter_size_header_edges(file_path, delimiter=delimiter)
    return _iter_plain_edges(file_path, delimiter=delimiter)


def load_edge_list(
        path: str | Path,
        directed: bool = False,
        delimiter: str | None = None,
        file_format: str = "auto",
) -> Graph:
    graph = Graph(directed=directed)
    for source, target in iter_edges(path, file_format=file_format, delimiter=delimiter):
        if source == target:
            continue
        graph.add_edge(source, target)
    return graph
