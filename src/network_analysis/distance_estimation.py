import json
import random
import time
from collections import deque
from dataclasses import dataclass
from pathlib import Path
from typing import Any

from .graph import Graph
from .metrics.basic import weakly_connected_components


def _component_nodes(graph: Graph) -> list[str]:
    components = weakly_connected_components(graph)
    return list(max(components, key=len, default=set()))


def bfs_distances_with_parents(
    graph: Graph,
    source: str,
    allowed_nodes: set[str] | None = None,
) -> tuple[dict[str, int], dict[str, str | None], dict[str, int]]:
    distances = {source: 0}
    parents: dict[str, str | None] = {source: None}
    depths = {source: 0}
    queue = deque([source])
    while queue:
        node = queue.popleft()
        for neighbor in graph.undirected_neighbors(node):
            if allowed_nodes is not None and neighbor not in allowed_nodes:
                continue
            if neighbor not in distances:
                distances[neighbor] = distances[node] + 1
                parents[neighbor] = node
                depths[neighbor] = depths[node] + 1
                queue.append(neighbor)
    return distances, parents, depths


def exact_distance(graph: Graph, source: str, target: str, allowed_nodes: set[str] | None = None) -> int | None:
    if source == target:
        return 0
    distances, _, _ = bfs_distances_with_parents(graph, source, allowed_nodes=allowed_nodes)
    return distances.get(target)


def select_landmarks(
    graph: Graph,
    component_nodes: list[str],
    count: int,
    strategy: str = "random",
    seed: int = 42,
) -> list[str]:
    if not component_nodes:
        return []
    count = min(count, len(component_nodes))
    randomizer = random.Random(seed)
    if strategy == "random":
        return randomizer.sample(component_nodes, count)
    if strategy == "highest_degree":
        return sorted(component_nodes, key=lambda node: graph.degree(node), reverse=True)[:count]
    if strategy == "coverage":
        allowed_nodes = set(component_nodes)
        first = max(component_nodes, key=lambda node: graph.degree(node))
        chosen = [first]
        distances, _, _ = bfs_distances_with_parents(graph, first, allowed_nodes=allowed_nodes)
        min_distances = {node: distances.get(node, float("inf")) for node in component_nodes}
        min_distances[first] = 0

        while len(chosen) < count:
            candidate = max(
                (node for node in component_nodes if node not in chosen),
                key=lambda node: min_distances.get(node, float("inf")),
                default=None,
            )
            if candidate is None:
                break
            chosen.append(candidate)
            candidate_distances, _, _ = bfs_distances_with_parents(graph, candidate, allowed_nodes=allowed_nodes)
            for node in component_nodes:
                current = min_distances.get(node, float("inf"))
                updated = candidate_distances.get(node, float("inf"))
                if updated < current:
                    min_distances[node] = updated
        return chosen
    raise ValueError(f"Unknown landmark strategy: {strategy}")


def _build_jump_table(parents: dict[str, str | None]) -> dict[str, list[str | None]]:
    max_power = 1
    node_count = max(1, len(parents))
    while (1 << max_power) <= node_count:
        max_power += 1

    jump: dict[str, list[str | None]] = {}
    for node, parent in parents.items():
        jump[node] = [parent] + [None] * (max_power - 1)

    for power in range(1, max_power):
        for node in jump:
            prev = jump[node][power - 1]
            jump[node][power] = jump[prev][power - 1] if prev is not None and prev in jump else None
    return jump


def _lift(node: str, delta: int, jump: dict[str, list[str | None]], depths: dict[str, int]) -> str | None:
    current: str | None = node
    bit = 0
    while delta > 0 and current is not None:
        if delta & 1:
            current = jump[current][bit]
        delta >>= 1
        bit += 1
    return current


def _lca(
    left: str,
    right: str,
    jump: dict[str, list[str | None]],
    depths: dict[str, int],
) -> str | None:
    if left not in depths or right not in depths:
        return None
    if depths[left] < depths[right]:
        left, right = right, left
    left_lifted = _lift(left, depths[left] - depths[right], jump, depths)
    if left_lifted is None:
        return None
    left = left_lifted
    if left == right:
        return left

    for power in reversed(range(len(next(iter(jump.values()))))):
        left_parent = jump[left][power]
        right_parent = jump[right][power]
        if left_parent != right_parent:
            if left_parent is not None:
                left = left_parent
            if right_parent is not None:
                right = right_parent
    return jump[left][0]


@dataclass
class LandmarkTree:
    root: str
    distances: dict[str, int]
    parents: dict[str, str | None]
    depths: dict[str, int]
    jump: dict[str, list[str | None]]

    def tree_distance(self, source: str, target: str) -> int | None:
        ancestor = _lca(source, target, self.jump, self.depths)
        if ancestor is None:
            return None
        return self.depths[source] + self.depths[target] - 2 * self.depths[ancestor]


@dataclass
class LandmarkIndex:
    algorithm: str
    component_nodes: list[str]
    landmarks: list[str]
    trees: dict[str, LandmarkTree]
    build_time_seconds: float

    def estimate_basic(self, source: str, target: str) -> int | None:
        best: int | None = None
        for landmark in self.landmarks:
            tree = self.trees[landmark]
            if source not in tree.distances or target not in tree.distances:
                continue
            estimate = tree.distances[source] + tree.distances[target]
            if best is None or estimate < best:
                best = estimate
        return best

    def estimate_lca(self, source: str, target: str) -> int | None:
        best: int | None = None
        for landmark in self.landmarks:
            tree = self.trees[landmark]
            if source not in tree.depths or target not in tree.depths:
                continue
            tree_distance = tree.tree_distance(source, target)
            if tree_distance is None:
                continue
            if best is None or tree_distance < best:
                best = tree_distance
        return best

    def estimate(self, source: str, target: str) -> int | None:
        if self.algorithm == "basic":
            return self.estimate_basic(source, target)
        if self.algorithm == "lca":
            basic_estimate = self.estimate_basic(source, target)
            lca_estimate = self.estimate_lca(source, target)
            candidates = [value for value in (basic_estimate, lca_estimate) if value is not None]
            return min(candidates) if candidates else None
        raise ValueError(f"Unknown algorithm: {self.algorithm}")

    def query(self, source: str, target: str, include_exact: bool = False, graph: Graph | None = None) -> dict[str, Any]:
        started = time.perf_counter()
        estimate = self.estimate(source, target)
        query_time = time.perf_counter() - started
        result: dict[str, Any] = {
            "source": source,
            "target": target,
            "estimate": estimate,
            "algorithm": self.algorithm,
            "landmarks": len(self.landmarks),
            "query_time_seconds": query_time,
        }
        if include_exact and graph is not None:
            exact = exact_distance(graph, source, target, allowed_nodes=set(self.component_nodes))
            result["exact_distance"] = exact
            result["absolute_error"] = None if estimate is None or exact is None else abs(estimate - exact)
        return result


def build_landmark_index(
    graph: Graph,
    landmark_count: int = 16,
    landmark_strategy: str = "random",
    algorithm: str = "basic",
    seed: int = 42,
    component_nodes: list[str] | None = None,
) -> LandmarkIndex:
    started = time.perf_counter()
    component_nodes = component_nodes or _component_nodes(graph)
    allowed_nodes = set(component_nodes)
    landmarks = select_landmarks(graph, component_nodes, landmark_count, strategy=landmark_strategy, seed=seed)
    trees: dict[str, LandmarkTree] = {}
    for landmark in landmarks:
        distances, parents, depths = bfs_distances_with_parents(graph, landmark, allowed_nodes=allowed_nodes)
        trees[landmark] = LandmarkTree(
            root=landmark,
            distances=distances,
            parents=parents,
            depths=depths,
            jump=_build_jump_table(parents),
        )
    build_time = time.perf_counter() - started
    return LandmarkIndex(
        algorithm=algorithm,
        component_nodes=component_nodes,
        landmarks=landmarks,
        trees=trees,
        build_time_seconds=build_time,
    )


def sample_vertex_pairs(component_nodes: list[str], pair_count: int, seed: int = 42) -> list[tuple[str, str]]:
    if len(component_nodes) < 2:
        return []
    randomizer = random.Random(seed)
    pairs: list[tuple[str, str]] = []
    for _ in range(pair_count):
        source = randomizer.choice(component_nodes)
        target = randomizer.choice(component_nodes)
        while target == source:
            target = randomizer.choice(component_nodes)
        pairs.append((source, target))
    return pairs


def evaluate_distance_estimator(
    graph: Graph,
    index: LandmarkIndex,
    pair_count: int = 200,
    seed: int = 42,
) -> dict[str, Any]:
    component_set = set(index.component_nodes)
    pairs = sample_vertex_pairs(index.component_nodes, pair_count, seed=seed)
    exact_time = 0.0
    estimate_time = 0.0
    measured: list[dict[str, Any]] = []
    exact_values: list[int] = []
    estimated_values: list[int] = []
    absolute_errors: list[int] = []

    for source, target in pairs:
        exact_started = time.perf_counter()
        exact = exact_distance(graph, source, target, allowed_nodes=component_set)
        exact_time += time.perf_counter() - exact_started

        estimate_started = time.perf_counter()
        estimate = index.estimate(source, target)
        estimate_time += time.perf_counter() - estimate_started

        if exact is None or estimate is None:
            continue

        error = abs(estimate - exact)
        exact_values.append(exact)
        estimated_values.append(estimate)
        absolute_errors.append(error)
        measured.append(
            {
                "source": source,
                "target": target,
                "exact_distance": exact,
                "estimated_distance": estimate,
                "absolute_error": error,
            }
        )

    correct = sum(1 for item in measured if item["absolute_error"] == 0)
    return {
        "algorithm": index.algorithm,
        "landmark_count": len(index.landmarks),
        "landmark_strategy": "unknown",
        "build_time_seconds": index.build_time_seconds,
        "evaluated_pairs": len(measured),
        "exact_match_ratio": (correct / len(measured)) if measured else 0.0,
        "mean_absolute_error": (sum(absolute_errors) / len(absolute_errors)) if absolute_errors else 0.0,
        "max_absolute_error": max(absolute_errors) if absolute_errors else 0,
        "mean_exact_distance": (sum(exact_values) / len(exact_values)) if exact_values else 0.0,
        "mean_estimated_distance": (sum(estimated_values) / len(estimated_values)) if estimated_values else 0.0,
        "exact_total_time_seconds": exact_time,
        "estimate_total_time_seconds": estimate_time,
        "speedup_vs_exact": (exact_time / estimate_time) if estimate_time > 0 else None,
        "sample_results": measured[: min(20, len(measured))],
    }


def compare_landmark_strategies(
    graph: Graph,
    algorithms: list[str],
    strategies: list[str],
    landmark_counts: list[int],
    pair_count: int = 200,
    seed: int = 42,
) -> dict[str, Any]:
    component_nodes = _component_nodes(graph)
    experiments: list[dict[str, Any]] = []
    for algorithm in algorithms:
        for strategy in strategies:
            for landmark_count in landmark_counts:
                index = build_landmark_index(
                    graph=graph,
                    landmark_count=landmark_count,
                    landmark_strategy=strategy,
                    algorithm=algorithm,
                    seed=seed,
                    component_nodes=component_nodes,
                )
                evaluation = evaluate_distance_estimator(graph, index, pair_count=pair_count, seed=seed)
                evaluation["landmark_strategy"] = strategy
                experiments.append(evaluation)

    best_accuracy = max(experiments, key=lambda item: (item["exact_match_ratio"], -item["mean_absolute_error"]), default=None)
    best_speed = max(experiments, key=lambda item: item["speedup_vs_exact"] or 0.0, default=None)
    return {
        "dataset_nodes_in_component": len(component_nodes),
        "algorithms": algorithms,
        "strategies": strategies,
        "landmark_counts": landmark_counts,
        "pair_count": pair_count,
        "experiments": experiments,
        "best_accuracy": best_accuracy,
        "best_speed": best_speed,
    }


def save_distance_experiment(output_dir: str | Path, report: dict[str, Any]) -> None:
    output_path = Path(output_dir)
    output_path.mkdir(parents=True, exist_ok=True)
    with (output_path / "distance_experiments.json").open("w", encoding="utf-8") as handle:
        json.dump(report, handle, indent=2, ensure_ascii=False)


def build_distance_markdown(report: dict[str, Any]) -> str:
    lines: list[str] = []
    lines.append("# Distance estimation experiments")
    lines.append("")
    lines.append(
        "| Algorithm | Strategy | Landmarks | Exact match ratio | MAE | Max error | Build time (s) | Speedup |"
    )
    lines.append("| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |")
    for experiment in report["experiments"]:
        speedup = experiment["speedup_vs_exact"] or 0.0
        lines.append(
            f"| {experiment['algorithm']} | {experiment['landmark_strategy']} | {experiment['landmark_count']} | "
            f"{experiment['exact_match_ratio']:.4f} | {experiment['mean_absolute_error']:.4f} | "
            f"{experiment['max_absolute_error']} | {experiment['build_time_seconds']:.4f} | {speedup:.2f} |"
        )
    lines.append("")
    if report.get("best_accuracy"):
        best = report["best_accuracy"]
        lines.append(
            f"- Best accuracy: `{best['algorithm']}` + `{best['landmark_strategy']}` + "
            f"{best['landmark_count']} landmarks, exact match ratio {best['exact_match_ratio']:.4f}, "
            f"MAE {best['mean_absolute_error']:.4f}."
        )
    if report.get("best_speed"):
        best = report["best_speed"]
        speedup = best["speedup_vs_exact"] or 0.0
        lines.append(
            f"- Best speedup: `{best['algorithm']}` + `{best['landmark_strategy']}` + "
            f"{best['landmark_count']} landmarks, speedup {speedup:.2f}x."
        )
    lines.append("")
    lines.append("- `basic` is usually faster at query time, but can overestimate distances.")
    lines.append("- `lca` uses landmark shortest-path trees and LCA queries, and is often more accurate.")
    lines.append("- More landmarks usually improve accuracy, but increase preprocessing time.")
    return "\n".join(lines) + "\n"
