from typing import Dict

from ..graph import Graph


def local_clustering_coefficients(graph: Graph) -> Dict[str, float]:
    coefficients: Dict[str, float] = {}
    for node in graph.nodes:
        neighbors = sorted(graph.undirected_neighbors(node))
        degree = len(neighbors)
        if degree < 2:
            coefficients[node] = 0.0
            continue
        links = 0
        for i, left in enumerate(neighbors):
            left_neighbors = graph.undirected_neighbors(left)
            for right in neighbors[i + 1:]:
                if right in left_neighbors:
                    links += 1
        coefficients[node] = (2 * links) / (degree * (degree - 1))
    return coefficients


def triangle_count(graph: Graph) -> int:
    count = 0
    for node in sorted(graph.nodes):
        neighbors = [neighbor for neighbor in graph.undirected_neighbors(node) if neighbor > node]
        for i, left in enumerate(neighbors):
            left_neighbors = graph.undirected_neighbors(left)
            for right in neighbors[i + 1:]:
                if right in left_neighbors:
                    count += 1
    return count


def average_clustering_coefficient(graph: Graph) -> float:
    coefficients = local_clustering_coefficients(graph)
    if not coefficients:
        return 0.0
    return sum(coefficients.values()) / len(coefficients)


def global_clustering_coefficient(graph: Graph) -> float:
    closed_triplets = triangle_count(graph) * 3
    total_triplets = 0
    for node in graph.nodes:
        degree = graph.degree(node)
        total_triplets += degree * (degree - 1) // 2
    if total_triplets == 0:
        return 0.0
    return closed_triplets / total_triplets
