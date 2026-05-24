from __future__ import annotations

import json
from pathlib import Path
from typing import Any

from .datasets import DATASETS
from .distance_estimation import build_distance_markdown, compare_landmark_strategies, save_distance_experiment
from .io_utils import load_edge_list


def run_distance_batch(
    root_dir: str | Path,
    algorithms: list[str],
    strategies: list[str],
    landmark_counts: list[int],
    pair_count: int,
    seed: int,
    dataset_names: list[str] | None = None,
    skip_existing: bool = True,
) -> dict[str, Any]:
    root = Path(root_dir)
    data_dir = root / "data"
    results_dir = root / "results" / "distance_batch"
    results_dir.mkdir(parents=True, exist_ok=True)

    dataset_results: list[dict[str, Any]] = []

    for dataset in DATASETS:
        if not dataset.get("enabled_for_distance", False):
            continue

        input_path = data_dir / dataset["filename"]
        if not input_path.exists():
            continue

        if dataset_names is not None and input_path.stem not in dataset_names:
            continue

        dataset_output_dir = results_dir / input_path.stem
        saved_json = dataset_output_dir / "distance_experiments.json"
        if skip_existing and saved_json.exists():
            with saved_json.open("r", encoding="utf-8") as handle:
                dataset_results.append(json.load(handle))
            continue

        graph = load_edge_list(
            input_path,
            directed=dataset["directed"],
            file_format=dataset.get("format", "auto"),
        )
        report = compare_landmark_strategies(
            graph=graph,
            algorithms=algorithms,
            strategies=strategies,
            landmark_counts=landmark_counts,
            pair_count=pair_count,
            seed=seed,
        )
        report["dataset"] = input_path.stem
        save_distance_experiment(dataset_output_dir, report)
        (dataset_output_dir / "distance_experiments.md").write_text(build_distance_markdown(report), encoding="utf-8")
        dataset_results.append(report)

    summary = build_distance_batch_summary(dataset_results)
    with (results_dir / "distance_batch_summary.json").open("w", encoding="utf-8") as handle:
        json.dump(summary, handle, indent=2, ensure_ascii=False)
    (results_dir / "distance_batch_summary.md").write_text(build_distance_batch_markdown(summary), encoding="utf-8")
    return summary


def build_distance_batch_summary(dataset_results: list[dict[str, Any]]) -> dict[str, Any]:
    flattened: list[dict[str, Any]] = []
    for dataset_report in dataset_results:
        for experiment in dataset_report["experiments"]:
            flattened.append({"dataset": dataset_report["dataset"], **experiment})

    best_accuracy = max(
        flattened,
        key=lambda item: (item["exact_match_ratio"], -item["mean_absolute_error"]),
        default=None,
    )
    best_speed = max(flattened, key=lambda item: item["speedup_vs_exact"] or 0.0, default=None)

    by_dataset = []
    for dataset_report in dataset_results:
        experiments = dataset_report["experiments"]
        dataset_best_accuracy = max(
            experiments,
            key=lambda item: (item["exact_match_ratio"], -item["mean_absolute_error"]),
            default=None,
        )
        dataset_best_speed = max(experiments, key=lambda item: item["speedup_vs_exact"] or 0.0, default=None)
        by_dataset.append(
            {
                "dataset": dataset_report["dataset"],
                "best_accuracy": dataset_best_accuracy,
                "best_speed": dataset_best_speed,
            }
        )

    observations = []
    if best_accuracy is not None:
        observations.append(
            f"Best accuracy overall: {best_accuracy['dataset']} / {best_accuracy['algorithm']} / "
            f"{best_accuracy['landmark_strategy']} / {best_accuracy['landmark_count']} landmarks, "
            f"exact match = {best_accuracy['exact_match_ratio']:.4f}, "
            f"MAE = {best_accuracy['mean_absolute_error']:.4f}."
        )
    if best_speed is not None:
        observations.append(
            f"Best speedup overall: {best_speed['dataset']} / {best_speed['algorithm']} / "
            f"{best_speed['landmark_strategy']} / {best_speed['landmark_count']} landmarks, "
            f"speedup = {(best_speed['speedup_vs_exact'] or 0.0):.2f}x."
        )
    observations.append("`basic` is usually faster at query time, but can overestimate distances.")
    observations.append("`bfs` often improves accuracy because it also uses landmark BFS-tree structure.")
    observations.append("More landmarks usually improve accuracy, but increase preprocessing time.")

    return {
        "dataset_count": len(dataset_results),
        "datasets": by_dataset,
        "all_experiments": flattened,
        "best_accuracy_overall": best_accuracy,
        "best_speed_overall": best_speed,
        "observations": observations,
    }


def build_distance_batch_markdown(summary: dict[str, Any]) -> str:
    lines: list[str] = []
    lines.append("# Distance estimation batch summary")
    lines.append("")
    lines.append("| Dataset | Best accuracy setup | Exact match | MAE | Best speed setup | Speedup |")
    lines.append("| --- | --- | ---: | ---: | --- | ---: |")
    for item in summary["datasets"]:
        best_acc = item["best_accuracy"]
        best_spd = item["best_speed"]
        acc_label = (
            f"{best_acc['algorithm']} / {best_acc['landmark_strategy']} / {best_acc['landmark_count']}"
            if best_acc else "-"
        )
        spd_label = (
            f"{best_spd['algorithm']} / {best_spd['landmark_strategy']} / {best_spd['landmark_count']}"
            if best_spd else "-"
        )
        acc_ratio = f"{best_acc['exact_match_ratio']:.4f}" if best_acc else "-"
        acc_mae = f"{best_acc['mean_absolute_error']:.4f}" if best_acc else "-"
        spd_value = (
            f"{best_spd['speedup_vs_exact']:.2f}"
            if best_spd and best_spd["speedup_vs_exact"] is not None
            else "-"
        )
        lines.append(
            f"| {item['dataset']} | {acc_label} | {acc_ratio} | {acc_mae} | {spd_label} | {spd_value} |"
        )
    lines.append("")
    lines.append("## Conclusions")
    lines.append("")
    for observation in summary["observations"]:
        lines.append(f"- {observation}")
    return "\n".join(lines) + "\n"
