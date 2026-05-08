import argparse
import json
from pathlib import Path

from .distance_estimation import (
    build_landmark_index,
    build_distance_markdown,
    compare_landmark_strategies,
    save_distance_experiment,
)
from .io_utils import detect_directed_from_name, load_edge_list


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Часть 2: оценка расстояний между вершинами методом landmarks.")
    parser.add_argument("input_path", help="Путь к датасету.")
    parser.add_argument("--output-dir", default="results/distance_report", help="Куда сохранять результаты.")
    parser.add_argument("--directed", action="store_true", help="Считать граф ориентированным.")
    parser.add_argument("--auto-directed", action="store_true", help="Определять ориентированность по имени файла.")
    parser.add_argument("--delimiter", default=None, help="Разделитель для edge list.")
    parser.add_argument("--format", default="auto", choices=["auto", "edge_list", "csv", "mtx"], help="Формат входного файла.")
    parser.add_argument("--algorithm", default="both", choices=["basic", "bfs", "both"], help="Какой алгоритм тестировать.")
    parser.add_argument(
        "--strategies",
        default="random,highest_degree,coverage",
        help="Стратегии выбора landmarks через запятую.",
    )
    parser.add_argument(
        "--landmark-counts",
        default="8,16,32",
        help="Количество landmarks через запятую.",
    )
    parser.add_argument("--source", default=None, help="Вершина-источник для точечного запроса.")
    parser.add_argument("--target", default=None, help="Вершина-приемник для точечного запроса.")
    parser.add_argument("--query-landmarks", type=int, default=16, help="Число landmarks для точечного запроса.")
    parser.add_argument(
        "--query-strategy",
        default="highest_degree",
        choices=["random", "highest_degree", "coverage"],
        help="Стратегия выбора landmarks для точечного запроса.",
    )
    parser.add_argument("--pair-count", type=int, default=200, help="Сколько пар вершин брать для проверки.")
    parser.add_argument("--seed", type=int, default=42, help="Seed для воспроизводимости.")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    directed = args.directed or (args.auto_directed and detect_directed_from_name(args.input_path))
    graph = load_edge_list(
        args.input_path,
        directed=directed,
        delimiter=args.delimiter,
        file_format=args.format,
    )

    if args.source is not None and args.target is not None:
        algorithm = "bfs" if args.algorithm == "both" else args.algorithm
        index = build_landmark_index(
            graph=graph,
            landmark_count=args.query_landmarks,
            landmark_strategy=args.query_strategy,
            algorithm=algorithm,
            seed=args.seed,
        )
        result = index.query(args.source, args.target, include_exact=True, graph=graph)
        print(json.dumps(result, indent=2, ensure_ascii=False))
        return 0

    algorithms = ["basic", "bfs"] if args.algorithm == "both" else [args.algorithm]
    strategies = [item.strip() for item in args.strategies.split(",") if item.strip()]
    landmark_counts = [int(item.strip()) for item in args.landmark_counts.split(",") if item.strip()]

    report = compare_landmark_strategies(
        graph=graph,
        algorithms=algorithms,
        strategies=strategies,
        landmark_counts=landmark_counts,
        pair_count=args.pair_count,
        seed=args.seed,
    )
    save_distance_experiment(args.output_dir, report)
    (Path(args.output_dir) / "distance_experiments.md").write_text(build_distance_markdown(report), encoding="utf-8")
    print(json.dumps(report, indent=2, ensure_ascii=False))
    return 0
