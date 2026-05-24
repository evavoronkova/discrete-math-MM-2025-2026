from __future__ import annotations

import argparse
import json
from pathlib import Path

from .io_utils import detect_directed_from_name, load_edge_list
from .report import build_report


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Анализ структуры сети для задания 1.")
    parser.add_argument("input_path", help="Путь к файлу со списком ребер.")
    parser.add_argument("--output-dir", default="results/report", help="Куда сохранять результаты.")
    parser.add_argument("--directed", action="store_true", help="Считать граф ориентированным.")
    parser.add_argument("--auto-directed", action="store_true", help="Определять ориентированность по имени файла.")
    parser.add_argument("--delimiter", default=None, help="Разделитель в файле ребер.")
    parser.add_argument(
        "--format",
        default="auto",
        choices=["auto", "edge_list", "size_header_edge_list", "csv", "mtx"],
        help="Формат входного файла.",
    )
    parser.add_argument("--distance-sample-size", type=int, default=500, help="Размер выборки случайных пар.")
    parser.add_argument("--snowball-size", type=int, default=500, help="Размер подграфа snowball.")
    parser.add_argument("--attack-steps", default="0,5,10,15,20,25,30,40,50", help="Проценты удаления узлов.")
    parser.add_argument("--seed", type=int, default=42, help="Seed для случайных выборок.")
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
    attack_steps = [float(item.strip()) for item in args.attack_steps.split(",") if item.strip()]
    report = build_report(
        graph=graph,
        output_dir=Path(args.output_dir),
        distance_sample_size=args.distance_sample_size,
        snowball_size=args.snowball_size,
        attack_steps=attack_steps,
        seed=args.seed,
    )
    print(json.dumps(report, indent=2, ensure_ascii=False))
    return 0
