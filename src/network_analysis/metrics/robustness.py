import random
from typing import Dict, Iterable, List

from ..graph import Graph
from .basic import largest_component_share, weakly_connected_components


def largest_weak_component_fraction(graph: Graph) -> float:
    components = weakly_connected_components(graph)
    return largest_component_share(components, graph.node_count())


def _percent_to_count(total: int, percent: float) -> int:
    return max(0, min(total, round(total * percent / 100.0)))


def random_attack(graph: Graph, percentages: Iterable[float], seed: int = 42) -> List[Dict[str, float]]:
    randomizer = random.Random(seed)
    nodes = list(graph.nodes)
    results: List[Dict[str, float]] = []
    for percent in percentages:
        remove_count = _percent_to_count(len(nodes), percent)
        removed = randomizer.sample(nodes, remove_count) if remove_count else []
        attacked_graph = graph.remove_nodes(removed)
        results.append(
            {
                "removed_percent": float(percent),
                "remaining_nodes": attacked_graph.node_count(),
                "largest_weak_component_share": largest_weak_component_fraction(attacked_graph),
            }
        )
    return results


def targeted_attack(graph: Graph, percentages: Iterable[float]) -> List[Dict[str, float]]:
    nodes_sorted = sorted(graph.nodes, key=lambda node: graph.degree(node), reverse=True)
    results: List[Dict[str, float]] = []
    for percent in percentages:
        remove_count = _percent_to_count(len(nodes_sorted), percent)
        removed = nodes_sorted[:remove_count]
        attacked_graph = graph.remove_nodes(removed)
        results.append(
            {
                "removed_percent": float(percent),
                "remaining_nodes": attacked_graph.node_count(),
                "largest_weak_component_share": largest_weak_component_fraction(attacked_graph),
            }
        )
    return results
