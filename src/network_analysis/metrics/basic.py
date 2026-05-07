from collections import deque
from statistics import mean
from typing import Dict, Iterable, List, Set

from ..graph import Graph


def density(graph: Graph) -> float:
    n = graph.node_count()
    if n < 2:
        return 0.0
    max_edges = n * (n - 1) if graph.directed else n * (n - 1) / 2
    return graph.edge_count() / max_edges


def weakly_connected_components(graph: Graph) -> List[Set[str]]:
    visited: Set[str] = set()
    components: List[Set[str]] = []
    for start in graph.nodes:
        if start in visited:
            continue
        component = set()
        queue = deque([start])
        visited.add(start)
        while queue:
            node = queue.popleft()
            component.add(node)
            for neighbor in graph.undirected_neighbors(node):
                if neighbor not in visited:
                    visited.add(neighbor)
                    queue.append(neighbor)
        components.append(component)
    return components


def _dfs_order(graph: Graph, start: str, visited: Set[str], order: List[str]) -> None:
    stack = [(start, False)]
    while stack:
        node, expanded = stack.pop()
        if expanded:
            order.append(node)
            continue
        if node in visited:
            continue
        visited.add(node)
        stack.append((node, True))
        for neighbor in graph.out_adj[node]:
            if neighbor not in visited:
                stack.append((neighbor, False))


def strongly_connected_components(graph: Graph) -> List[Set[str]]:
    if not graph.directed:
        return weakly_connected_components(graph)

    visited: Set[str] = set()
    order: List[str] = []
    for node in graph.nodes:
        if node not in visited:
            _dfs_order(graph, node, visited, order)

    reversed_graph = graph.reversed()
    visited.clear()
    components: List[Set[str]] = []
    for start in reversed(order):
        if start in visited:
            continue
        component = set()
        stack = [start]
        visited.add(start)
        while stack:
            node = stack.pop()
            component.add(node)
            for neighbor in reversed_graph.out_adj[node]:
                if neighbor not in visited:
                    visited.add(neighbor)
                    stack.append(neighbor)
        components.append(component)
    return components


def degree_stats(graph: Graph) -> Dict[str, float]:
    degrees = [graph.degree(node) for node in graph.nodes]
    if not degrees:
        return {"min": 0, "max": 0, "mean": 0.0}
    return {
        "min": min(degrees),
        "max": max(degrees),
        "mean": mean(degrees),
    }


def degree_distribution(graph: Graph) -> Dict[int, float]:
    degrees = [graph.degree(node) for node in graph.nodes]
    if not degrees:
        return {}
    total = len(degrees)
    distribution: Dict[int, float] = {}
    for degree in degrees:
        distribution[degree] = distribution.get(degree, 0.0) + 1.0 / total
    return dict(sorted(distribution.items()))


def largest_component_share(components: Iterable[Set[str]], total_nodes: int) -> float:
    if total_nodes == 0:
        return 0.0
    largest = max((len(component) for component in components), default=0)
    return largest / total_nodes
