from __future__ import annotations

import argparse
import json
from pathlib import Path

from .distance_estimation import build_landmark_index
from .io_utils import detect_directed_from_name, load_edge_list
from .metrics.basic import degree_stats, weakly_connected_components
from .metrics.clustering import local_clustering_coefficients
from .metrics.robustness import random_attack, targeted_attack
from .report import build_report


def _load_graph(args: argparse.Namespace):
    directed = args.directed or (getattr(args, "auto_directed", False) and detect_directed_from_name(args.input_path))
    return load_edge_list(
        args.input_path,
        directed=directed,
        delimiter=getattr(args, "delimiter", None),
        file_format=getattr(args, "format", "auto"),
    )


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="Demo-интерфейс для защиты проекта.")
    parser.add_argument("input_path", help="Путь к датасету.")
    parser.add_argument("--directed", action="store_true", help="Считать граф ориентированным.")
    parser.add_argument("--auto-directed", action="store_true", help="Определять ориентированность по имени файла.")
    parser.add_argument("--delimiter", default=None, help="Разделитель для edge list.")
    parser.add_argument("--format", default="auto", choices=["auto", "edge_list", "size_header_edge_list", "csv", "mtx"], help="Формат входного файла.")

    subparsers = parser.add_subparsers(dest="command", required=True)

    summary_parser = subparsers.add_parser("summary", help="Показать краткую сводку по сети.")
    summary_parser.add_argument("--output-dir", default=None, help="Если указан, сохранить полный report.")

    pair_parser = subparsers.add_parser("distance", help="Оценить расстояние между двумя вершинами.")
    pair_parser.add_argument("--source", required=True, help="Вершина-источник.")
    pair_parser.add_argument("--target", required=True, help="Вершина-приемник.")
    pair_parser.add_argument("--algorithm", default="lca", choices=["basic", "lca"], help="Алгоритм оценки.")
    pair_parser.add_argument("--landmarks", type=int, default=16, help="Количество landmarks.")
    pair_parser.add_argument("--strategy", default="highest_degree", choices=["random", "highest_degree", "coverage"])
    pair_parser.add_argument("--seed", type=int, default=42)

    local_cc_parser = subparsers.add_parser("local-cc", help="Вычислить локальный кластерный коэффициент вершины.")
    local_cc_parser.add_argument("--node", required=True, help="Идентификатор вершины.")

    robustness_parser = subparsers.add_parser("robustness", help="Показать устойчивость сети при удалении вершин.")
    robustness_parser.add_argument("--mode", required=True, choices=["random", "targeted"])
    robustness_parser.add_argument("--steps", default="0,5,10,15,20,25,30,40,50")
    robustness_parser.add_argument("--seed", type=int, default=42)

    return parser


def main() -> int:
    parser = build_parser()
    args = parser.parse_args()
    graph = _load_graph(args)

    if args.command == "summary":
        weak_components = weakly_connected_components(graph)
        largest_weak = max((len(component) for component in weak_components), default=0)
        result = {
            "nodes": graph.node_count(),
            "edges": graph.edge_count(),
            "directed": graph.directed,
            "weak_components": len(weak_components),
            "largest_weak_component_size": largest_weak,
            "degree_stats": degree_stats(graph),
        }
        if args.output_dir:
            build_report(graph, Path(args.output_dir))
            result["saved_report_dir"] = str(Path(args.output_dir))
        print(json.dumps(result, indent=2, ensure_ascii=False))
        return 0

    if args.command == "distance":
        index = build_landmark_index(
            graph=graph,
            landmark_count=args.landmarks,
            landmark_strategy=args.strategy,
            algorithm=args.algorithm,
            seed=args.seed,
        )
        result = index.query(args.source, args.target, include_exact=True, graph=graph)
        print(json.dumps(result, indent=2, ensure_ascii=False))
        return 0

    if args.command == "local-cc":
        coefficients = local_clustering_coefficients(graph)
        result = {
            "node": args.node,
            "local_clustering_coefficient": coefficients.get(args.node),
            "degree": graph.degree(args.node) if args.node in graph.out_adj else None,
        }
        print(json.dumps(result, indent=2, ensure_ascii=False))
        return 0

    if args.command == "robustness":
        steps = [float(item.strip()) for item in args.steps.split(",") if item.strip()]
        if args.mode == "random":
            result = random_attack(graph, steps, seed=args.seed)
        else:
            result = targeted_attack(graph, steps)
        print(json.dumps(result, indent=2, ensure_ascii=False))
        return 0

    return 0
