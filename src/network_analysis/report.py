import csv
import json
from pathlib import Path
from typing import Iterable

from .graph import Graph
from .metrics.basic import (
    degree_distribution,
    degree_stats,
    density,
    largest_component_share,
    strongly_connected_components,
    weakly_connected_components,
)
from .metrics.clustering import (
    average_clustering_coefficient,
    global_clustering_coefficient,
    local_clustering_coefficients,
    triangle_count,
)
from .metrics.distance import (
    double_sweep_diameter_estimate,
    exact_diameter,
    sample_pair_distances,
    snowball_sample,
    summarize_distances,
)
from .metrics.robustness import random_attack, targeted_attack
from .plotting import plot_degree_distribution


def _write_csv(path: Path, rows: Iterable[dict]) -> None:
    rows = list(rows)
    if not rows:
        return
    with path.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=list(rows[0].keys()))
        writer.writeheader()
        writer.writerows(rows)


def build_report(
        graph: Graph,
        output_dir: str | Path,
        distance_sample_size: int = 500,
        snowball_size: int = 500,
        diameter_vertex_samples: list[int] | None = None,
        exact_diameter_max_nodes: int = 2000,
        attack_steps: list[float] | None = None,
        seed: int = 42,
) -> dict:
    output_path = Path(output_dir)
    output_path.mkdir(parents=True, exist_ok=True)
    attack_steps = attack_steps or [0, 5, 10, 15, 20, 25, 30, 40, 50]

    weak_components = weakly_connected_components(graph)
    largest_weak = max(weak_components, key=len, default=set())
    weak_subgraph = graph.induced_subgraph(largest_weak)

    strong_components = strongly_connected_components(graph) if graph.directed else []
    degree_dist = degree_distribution(graph)
    local_cc = local_clustering_coefficients(weak_subgraph)

    double_sweep = double_sweep_diameter_estimate(graph, list(largest_weak), seed=seed) if largest_weak else {}
    random_distances = sample_pair_distances(graph, list(largest_weak), sample_size=distance_sample_size, seed=seed)
    snowball_nodes = snowball_sample(graph, list(largest_weak), target_size=snowball_size, seed=seed)
    snowball_distances = sample_pair_distances(graph, snowball_nodes, sample_size=distance_sample_size, seed=seed)
    sample_sizes = diameter_vertex_samples or [200, 500, 1000]
    varied_diameter: list[dict] = []
    for sample_size in sample_sizes:
        current_size = max(2, min(sample_size, len(largest_weak)))
        if current_size > len(largest_weak):
            continue
        sampled_nodes = snowball_sample(graph, list(largest_weak), target_size=current_size, seed=seed)
        if len(sampled_nodes) < 2:
            continue
        sampled_subgraph = graph.induced_subgraph(sampled_nodes)
        varied_diameter.append(
            {
                "requested_sample_size": sample_size,
                "actual_sample_size": len(sampled_nodes),
                "double_sweep": double_sweep_diameter_estimate(sampled_subgraph, sampled_nodes, seed=seed),
            }
        )
    exact_diameter_result = exact_diameter(
        graph,
        list(largest_weak),
        max_nodes_for_exact=exact_diameter_max_nodes,
    )

    report = {
        "graph": {
            "directed": graph.directed,
            "nodes": graph.node_count(),
            "edges": graph.edge_count(),
            "density": density(graph),
        },
        "components": {
            "weak_count": len(weak_components),
            "largest_weak_share": largest_component_share(weak_components, graph.node_count()),
            "strong_count": len(strong_components) if graph.directed else None,
            "largest_strong_share": (
                largest_component_share(strong_components, graph.node_count()) if graph.directed else None
            ),
        },
        "degrees": {
            **degree_stats(graph),
            "distribution": degree_dist,
        },
        "clustering": {
            "triangles": triangle_count(graph),
            "average_clustering": average_clustering_coefficient(graph),
            "global_clustering": global_clustering_coefficient(graph),
            "largest_weak_average_clustering": (
                sum(local_cc.values()) / len(local_cc) if local_cc else 0.0
            ),
        },
        "distance_estimates": {
            "double_sweep": double_sweep,
            "double_sweep_varied_vertex_samples": varied_diameter,
            "exact_diameter_small_graph": exact_diameter_result,
            "random_pairs": summarize_distances(random_distances),
            "snowball_pairs": summarize_distances(snowball_distances),
            "snowball_sample_size": len(snowball_nodes),
        },
    }

    plot_degree_distribution(degree_dist, output_path / "degree_distribution.png", loglog=False)
    plot_degree_distribution(degree_dist, output_path / "degree_distribution_loglog.png", loglog=True)
    _write_csv(output_path / "robustness_random.csv", random_attack(graph, attack_steps, seed=seed))
    _write_csv(output_path / "robustness_targeted.csv", targeted_attack(graph, attack_steps))

    with (output_path / "summary.json").open("w", encoding="utf-8") as handle:
        json.dump(report, handle, indent=2, ensure_ascii=False)

    return report
