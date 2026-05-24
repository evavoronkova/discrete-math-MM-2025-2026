import math
import random
from collections import deque
from typing import Dict, Iterable, List, Sequence, Tuple

from ..graph import Graph


def bfs_distances(graph: Graph, source: str, allowed_nodes: set[str] | None = None) -> Dict[str, int]:
    distances = {source: 0}
    queue = deque([source])
    while queue:
        node = queue.popleft()
        for neighbor in graph.undirected_neighbors(node):
            if allowed_nodes is not None and neighbor not in allowed_nodes:
                continue
            if neighbor not in distances:
                distances[neighbor] = distances[node] + 1
                queue.append(neighbor)
    return distances


def eccentricity(graph: Graph, source: str, allowed_nodes: set[str] | None = None) -> Tuple[str, int]:
    distances = bfs_distances(graph, source, allowed_nodes=allowed_nodes)
    farthest_node = max(distances, key=distances.__getitem__)
    return farthest_node, distances[farthest_node]


def double_sweep_diameter_estimate(graph: Graph, component_nodes: Iterable[str], seed: int = 42) -> Dict[str, object]:
    component = list(component_nodes)
    if not component:
        return {
            "start": None,
            "first_end": None,
            "second_end": None,
            "diameter_estimate": 0,
        }
    randomizer = random.Random(seed)
    start = randomizer.choice(component)
    first_end, _ = eccentricity(graph, start, allowed_nodes=set(component))
    second_end, diameter_estimate = eccentricity(graph, first_end, allowed_nodes=set(component))
    return {
        "start": start,
        "first_end": first_end,
        "second_end": second_end,
        "diameter_estimate": diameter_estimate,
    }


def exact_diameter(
    graph: Graph,
    component_nodes: Sequence[str],
    max_nodes_for_exact: int = 2000,
) -> Dict[str, object]:
    component = list(component_nodes)
    node_count = len(component)
    if node_count == 0:
        return {
            "computed": True,
            "reason": "empty_component",
            "nodes": 0,
            "diameter": 0,
            "endpoints": [None, None],
        }
    if node_count > max_nodes_for_exact:
        return {
            "computed": False,
            "reason": "component_too_large",
            "nodes": node_count,
            "max_nodes_for_exact": max_nodes_for_exact,
            "diameter": None,
            "endpoints": None,
        }

    allowed_nodes = set(component)
    diameter = 0
    endpoints: tuple[str | None, str | None] = (component[0], component[0])
    for source in component:
        distances = bfs_distances(graph, source, allowed_nodes=allowed_nodes)
        if not distances:
            continue
        target, distance = max(distances.items(), key=lambda item: item[1])
        if distance > diameter:
            diameter = distance
            endpoints = (source, target)

    return {
        "computed": True,
        "reason": "ok",
        "nodes": node_count,
        "diameter": diameter,
        "endpoints": list(endpoints),
    }


def sample_pair_distances(
    graph: Graph,
    component_nodes: Sequence[str],
    sample_size: int = 500,
    seed: int = 42,
) -> List[int]:
    if len(component_nodes) < 2:
        return []
    randomizer = random.Random(seed)
    distances: List[int] = []
    for _ in range(sample_size):
        source = randomizer.choice(component_nodes)
        target = randomizer.choice(component_nodes)
        while target == source and len(component_nodes) > 1:
            target = randomizer.choice(component_nodes)
        dist_map = bfs_distances(graph, source, allowed_nodes=set(component_nodes))
        if target in dist_map:
            distances.append(dist_map[target])
    return distances


def snowball_sample(graph: Graph, component_nodes: Sequence[str], target_size: int = 500, seed: int = 42) -> List[str]:
    if not component_nodes:
        return []
    component_set = set(component_nodes)
    randomizer = random.Random(seed)
    start = randomizer.choice(component_nodes)
    seed_nodes = [start] + list(graph.undirected_neighbors(start))[:2]
    sample = {node for node in seed_nodes if node in component_set}
    frontier = deque(sample)
    while frontier and len(sample) < target_size:
        node = frontier.popleft()
        for neighbor in graph.undirected_neighbors(node):
            if neighbor in component_set and neighbor not in sample:
                sample.add(neighbor)
                frontier.append(neighbor)
                if len(sample) >= target_size:
                    break
    return list(sample)


def percentile(values: Sequence[int], p: float) -> float:
    if not values:
        return 0.0
    sorted_values = sorted(values)
    index = math.ceil(p * len(sorted_values)) - 1
    index = max(0, min(index, len(sorted_values) - 1))
    return float(sorted_values[index])


def summarize_distances(distances: Sequence[int]) -> Dict[str, float]:
    if not distances:
        return {"count": 0, "diameter_estimate": 0.0, "p90_distance": 0.0, "mean_distance": 0.0}
    return {
        "count": len(distances),
        "diameter_estimate": float(max(distances)),
        "p90_distance": percentile(distances, 0.90),
        "mean_distance": sum(distances) / len(distances),
    }
