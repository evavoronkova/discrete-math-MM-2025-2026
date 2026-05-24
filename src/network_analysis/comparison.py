from __future__ import annotations

import csv
import json
from dataclasses import dataclass
from pathlib import Path
from typing import Any


EXCLUDED_RESULT_DIR_NAMES = {
    "report",
    "sample_report",
    "sample_digraph_report",
}


@dataclass
class DatasetComparison:
    name: str
    summary_path: Path
    metrics: dict[str, Any]


def _read_json(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def _read_last_robustness_value(path: Path) -> dict[str, float] | None:
    if not path.exists():
        return None
    with path.open("r", encoding="utf-8", newline="") as handle:
        rows = list(csv.DictReader(handle))
    if not rows:
        return None
    last = rows[-1]
    return {
        "removed_percent": float(last["removed_percent"]),
        "remaining_nodes": float(last["remaining_nodes"]),
        "largest_weak_component_share": float(last["largest_weak_component_share"]),
    }


def _dataset_entry(result_dir: Path) -> DatasetComparison:
    summary = _read_json(result_dir / "summary.json")
    metrics = {
        "directed": summary["graph"]["directed"],
        "nodes": summary["graph"]["nodes"],
        "edges": summary["graph"]["edges"],
        "density": summary["graph"]["density"],
        "weak_count": summary["components"]["weak_count"],
        "largest_weak_share": summary["components"]["largest_weak_share"],
        "strong_count": summary["components"]["strong_count"],
        "largest_strong_share": summary["components"]["largest_strong_share"],
        "min_degree": summary["degrees"]["min"],
        "max_degree": summary["degrees"]["max"],
        "mean_degree": summary["degrees"]["mean"],
        "triangles": summary["clustering"]["triangles"],
        "average_clustering": summary["clustering"]["average_clustering"],
        "global_clustering": summary["clustering"]["global_clustering"],
        "largest_weak_average_clustering": summary["clustering"]["largest_weak_average_clustering"],
        "double_sweep_diameter": summary["distance_estimates"]["double_sweep"].get("diameter_estimate", 0),
        "random_pairs_p90": summary["distance_estimates"]["random_pairs"]["p90_distance"],
        "random_pairs_diameter": summary["distance_estimates"]["random_pairs"]["diameter_estimate"],
        "random_pairs_mean": summary["distance_estimates"]["random_pairs"]["mean_distance"],
        "snowball_p90": summary["distance_estimates"]["snowball_pairs"]["p90_distance"],
        "snowball_diameter": summary["distance_estimates"]["snowball_pairs"]["diameter_estimate"],
        "snowball_mean": summary["distance_estimates"]["snowball_pairs"]["mean_distance"],
        "random_attack_last": _read_last_robustness_value(result_dir / "robustness_random.csv"),
        "targeted_attack_last": _read_last_robustness_value(result_dir / "robustness_targeted.csv"),
    }
    return DatasetComparison(name=result_dir.name, summary_path=result_dir / "summary.json", metrics=metrics)


def load_available_comparisons(results_dir: str | Path) -> list[DatasetComparison]:
    base = Path(results_dir)
    comparisons: list[DatasetComparison] = []
    if not base.exists():
        return comparisons
    for item in sorted(base.iterdir()):
        if not item.is_dir():
            continue
        if item.name in EXCLUDED_RESULT_DIR_NAMES:
            continue
        if item.name.endswith("_test"):
            continue
        summary_path = item / "summary.json"
        if summary_path.exists():
            comparisons.append(_dataset_entry(item))
    return comparisons


def _metric_ranking(comparisons: list[DatasetComparison], key: str, reverse: bool = True) -> list[dict[str, Any]]:
    ranked = sorted(comparisons, key=lambda item: item.metrics[key], reverse=reverse)
    return [{"dataset": item.name, "value": item.metrics[key]} for item in ranked]


def _robustness_ranking(comparisons: list[DatasetComparison], attack_key: str) -> list[dict[str, Any]]:
    ranked = sorted(
        [item for item in comparisons if item.metrics[attack_key] is not None],
        key=lambda item: item.metrics[attack_key]["largest_weak_component_share"],
        reverse=True,
    )
    return [
        {
            "dataset": item.name,
            "remaining_share": item.metrics[attack_key]["largest_weak_component_share"],
            "removed_percent": item.metrics[attack_key]["removed_percent"],
        }
        for item in ranked
    ]


def _build_observations(comparisons: list[DatasetComparison]) -> list[str]:
    observations: list[str] = []
    if not comparisons:
        return observations

    largest = max(comparisons, key=lambda item: item.metrics["nodes"])
    densest = max(comparisons, key=lambda item: item.metrics["density"])
    most_clustered = max(comparisons, key=lambda item: item.metrics["average_clustering"])
    widest = max(comparisons, key=lambda item: item.metrics["double_sweep_diameter"])
    most_fragmented = max(comparisons, key=lambda item: item.metrics["weak_count"])

    random_candidates = [item for item in comparisons if item.metrics["random_attack_last"] is not None]
    targeted_candidates = [item for item in comparisons if item.metrics["targeted_attack_last"] is not None]

    observations.append(f"Largest network by node count: {largest.name} ({largest.metrics['nodes']} nodes).")
    observations.append(
        f"Highest density: {densest.name} ({densest.metrics['density']:.6f}); "
        "this indicates a larger share of possible local ties."
    )
    observations.append(
        f"Highest average clustering coefficient: {most_clustered.name} "
        f"({most_clustered.metrics['average_clustering']:.4f})."
    )
    observations.append(
        f"Largest double-sweep diameter estimate: {widest.name} "
        f"({widest.metrics['double_sweep_diameter']})."
    )
    observations.append(
        f"Most fragmented network by weak component count: {most_fragmented.name} "
        f"({most_fragmented.metrics['weak_count']} components)."
    )

    if random_candidates:
        most_resilient_random = max(
            random_candidates,
            key=lambda item: item.metrics["random_attack_last"]["largest_weak_component_share"],
        )
        observations.append(
            f"Under random node removal, {most_resilient_random.name} keeps the largest weak component best "
            f"({most_resilient_random.metrics['random_attack_last']['largest_weak_component_share']:.4f} "
            f"after removing {most_resilient_random.metrics['random_attack_last']['removed_percent']:.0f}% of nodes)."
        )

    if targeted_candidates:
        least_resilient_targeted = min(
            targeted_candidates,
            key=lambda item: item.metrics["targeted_attack_last"]["largest_weak_component_share"],
        )
        observations.append(
            f"Under targeted removal by highest degree, {least_resilient_targeted.name} is damaged the most "
            f"(largest weak component share {least_resilient_targeted.metrics['targeted_attack_last']['largest_weak_component_share']:.4f})."
        )

    return observations


def build_comparison_summary(comparisons: list[DatasetComparison]) -> dict[str, Any]:
    return {
        "datasets": [
            {
                "name": item.name,
                "summary_path": str(item.summary_path),
                **item.metrics,
            }
            for item in comparisons
        ],
        "rankings": {
            "by_nodes": _metric_ranking(comparisons, "nodes"),
            "by_density": _metric_ranking(comparisons, "density"),
            "by_average_clustering": _metric_ranking(comparisons, "average_clustering"),
            "by_double_sweep_diameter": _metric_ranking(comparisons, "double_sweep_diameter"),
            "by_random_attack_resilience": _robustness_ranking(comparisons, "random_attack_last"),
            "by_targeted_attack_resilience": _robustness_ranking(comparisons, "targeted_attack_last"),
        },
        "observations": _build_observations(comparisons),
    }


def build_comparison_markdown(summary: dict[str, Any]) -> str:
    lines: list[str] = []
    lines.append("# Network comparison (task 1C)")
    lines.append("")
    lines.append("## Summary table")
    lines.append("")
    lines.append(
        "| Network | Directed | Nodes | Edges | Density | Weak components | Largest weak share | "
        "Mean degree | Avg clustering | Diameter (double sweep) | P90 |"
    )
    lines.append("| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |")
    for item in summary["datasets"]:
        lines.append(
            f"| {item['name']} | {'yes' if item['directed'] else 'no'} | {item['nodes']} | {item['edges']} | "
            f"{item['density']:.6f} | {item['weak_count']} | {item['largest_weak_share']:.4f} | "
            f"{item['mean_degree']:.4f} | {item['average_clustering']:.4f} | "
            f"{item['double_sweep_diameter']} | {item['random_pairs_p90']:.2f} |"
        )
    lines.append("")
    lines.append("## Observations")
    lines.append("")
    for observation in summary["observations"]:
        lines.append(f"- {observation}")
    lines.append("")
    lines.append("## Robustness")
    lines.append("")
    lines.append("The comparison uses the share of nodes in the largest weak component after the last removal step.")
    lines.append("")
    for item in summary["datasets"]:
        random_attack = item["random_attack_last"]
        targeted_attack = item["targeted_attack_last"]
        if random_attack is None or targeted_attack is None:
            continue
        lines.append(
            f"- {item['name']}: random removal -> {random_attack['largest_weak_component_share']:.4f}, "
            f"highest-degree removal -> {targeted_attack['largest_weak_component_share']:.4f}."
        )
    lines.append("")
    lines.append("## Interpretation")
    lines.append("")
    lines.append("- Higher density and clustering usually indicate denser local communities.")
    lines.append("- Larger diameter and P90 distance indicate longer paths between typical vertex pairs.")
    lines.append("- A sharp drop under targeted removal shows that the network depends on a small set of high-degree hubs.")
    return "\n".join(lines) + "\n"


def save_comparison_artifacts(results_dir: str | Path) -> dict[str, Any]:
    base = Path(results_dir)
    comparisons = load_available_comparisons(base)
    summary = build_comparison_summary(comparisons)

    summary_path = base / "comparison_summary.json"
    markdown_path = base / "comparison_report.md"

    with summary_path.open("w", encoding="utf-8") as handle:
        json.dump(summary, handle, indent=2, ensure_ascii=False)

    markdown_path.write_text(build_comparison_markdown(summary), encoding="utf-8")
    return summary
