# запуск и сравнение алгоритмов
import csv
import os
import random
import time
from typing import List

from experiments._io import graph_output_dir, log, open_report
from experiments.plot_results import plot_landmarks_results
from src.analysis import largest_cc_vertices
from src.graph import Graph
from src.landmarks import (
    LandmarksBasic,
    LandmarksSC,
    _GraphIndex,
    select_best_coverage_landmarks,
    select_degree_landmarks,
    select_random_landmarks,
)
from src.utils import bfs

DATASET_ROOT = "datasets"
OUTPUT_DIR = "results"
os.makedirs(OUTPUT_DIR, exist_ok=True)

# параметры экспериментов
KS = [5, 10, 20, 50]
STRATEGIES = ["random", "degree", "coverage"]
NUM_PAIRS = 100
COVERAGE_M = 200

CSV_HEADER = [
    "graph", "k", "strategy",
    "basic_prep_time", "sc_prep_time",
    "basic_query_time", "sc_query_time",
    "basic_mre", "sc_mre",
    "basic_exact_frac", "sc_exact_frac",
]

def load_graph(filepath: str) -> Graph:
    norm = os.path.normpath(filepath)
    directed = "directed" in norm.split(os.sep)
    return Graph.from_file(filepath, directed=directed)

def collect_all_files(root: str) -> List[str]:
    all_files = []
    for dirpath, _, filenames in os.walk(root):
        for fname in filenames:
            all_files.append(os.path.join(dirpath, fname))
    return all_files

def select_landmarks(index: _GraphIndex, k: int, strategy: str) -> List[int]:
    if strategy == "random":
        return select_random_landmarks(index, k)
    if strategy == "degree":
        return select_degree_landmarks(index, k)
    if strategy == "coverage":
        return select_best_coverage_landmarks(index, k, M=COVERAGE_M)
    raise ValueError(f"Unknown strategy: {strategy}")

def evaluate_accuracy(exact_dist, est_basic, est_sc):
    mre_basic_list, mre_sc_list = [], []
    exact_basic = exact_sc = valid = 0
    for true_d, e_b, e_s in zip(exact_dist, est_basic, est_sc):
        if true_d == -1:
            continue
        valid += 1
        if true_d == 0:
            mre_basic_list.append(0.0 if e_b == 0 else 1.0)
            mre_sc_list.append(0.0 if e_s == 0 else 1.0)
            if e_b == 0:
                exact_basic += 1
            if e_s == 0:
                exact_sc += 1
        else:
            if e_b == true_d:
                exact_basic += 1
            if e_s == true_d:
                exact_sc += 1
            mre_basic_list.append(abs(e_b - true_d) / true_d if e_b != -1 else 1.0)
            mre_sc_list.append(abs(e_s - true_d) / true_d if e_s != -1 else 1.0)
    if valid == 0:
        return 0.0, 0.0, 0.0, 0.0
    return (
        sum(mre_basic_list) / valid,
        sum(mre_sc_list) / valid,
        exact_basic / valid,
        exact_sc / valid,
    )

def process_graph(filepath: str, aggregated_csv: str) -> None:
    name = os.path.relpath(filepath, DATASET_ROOT).replace("/", "_").replace(".", "_")
    out_dir = graph_output_dir(OUTPUT_DIR, name)
    report_path = os.path.join(out_dir, "landmarks.txt")

    with open_report(report_path) as report:
        log(report, f"\n===== {name} =====")
        g = load_graph(filepath)
        # по заданию граф трактуется как неориентированный; делаем переход
        # один раз — тогда и _GraphIndex (CSR для landmarks), и эталонный
        # bfs работают на одном и том же неориентированном виде, и MRE
        # сравнивает сопоставимые величины
        g = g.to_undirected()

        lcc_verts = largest_cc_vertices(g)
        log(report, f"  Наибольшая компонента: {len(lcc_verts)} вершин")

        nodes_lcc = list(lcc_verts)
        if len(nodes_lcc) < 2:
            log(report, "  Недостаточно вершин для пар")
            return

        random.seed(40)
        pairs = []
        while len(pairs) < NUM_PAIRS:
            a = random.choice(nodes_lcc)
            b = random.choice(nodes_lcc)
            if a != b:
                pairs.append((a, b))

        log(report, "  Вычисление эталонных расстояний...")
        exact_dist = []
        t0 = time.time()
        for u, v in pairs:
            dist_u = bfs(g, u)
            exact_dist.append(dist_u[v] if v in dist_u else -1)
        log(report, f"  Эталонные расстояния: {time.time() - t0:.2f} с")

        log(report, "  Построение _GraphIndex...")
        t0 = time.time()
        index = _GraphIndex(g)
        log(report, f"  _GraphIndex: {time.time() - t0:.2f} с")

        basic_prep_times, sc_prep_times = {}, {}
        basic_query_times, sc_query_times = {}, {}
        basic_mre, sc_mre = {}, {}
        basic_exact_frac, sc_exact_frac = {}, {}
        rows_to_write = []

        for k in KS:
            for strat in STRATEGIES:
                log(report, f"  k={k}, стратегия {strat}")
                landmarks = select_landmarks(index, k, strat)

                t0 = time.time()
                lb = LandmarksBasic(g, landmarks, _index=index)
                prep_basic = time.time() - t0
                t0 = time.time()
                est_basic = [lb.estimate(u, v) for u, v in pairs]
                query_basic = time.time() - t0

                t0 = time.time()
                lsc = LandmarksSC(g, landmarks, _index=index)
                prep_sc = time.time() - t0
                t0 = time.time()
                est_sc = [lsc.estimate(u, v) for u, v in pairs]
                query_sc = time.time() - t0

                avg_mre_basic, avg_mre_sc, frac_basic, frac_sc = evaluate_accuracy(
                    exact_dist, est_basic, est_sc
                )
                log(report,
                    f"    Basic: MRE={avg_mre_basic:.4f}, точных={frac_basic:.2f}, "
                    f"prep={prep_basic:.2f}с, query={query_basic:.3f}с")
                log(report,
                    f"    SC:    MRE={avg_mre_sc:.4f}, точных={frac_sc:.2f}, "
                    f"prep={prep_sc:.2f}с, query={query_sc:.3f}с")

                key = (k, strat)
                basic_prep_times[key] = prep_basic
                sc_prep_times[key] = prep_sc
                basic_query_times[key] = query_basic
                sc_query_times[key] = query_sc
                basic_mre[key] = avg_mre_basic
                sc_mre[key] = avg_mre_sc
                basic_exact_frac[key] = frac_basic
                sc_exact_frac[key] = frac_sc

                rows_to_write.append([
                    name, k, strat,
                    prep_basic, prep_sc,
                    query_basic, query_sc,
                    avg_mre_basic, avg_mre_sc,
                    frac_basic, frac_sc,
                ])

        # дозапись в общий aggregated CSV (для сравнения между графами)
        with open(aggregated_csv, "a", newline="") as f:
            csv.writer(f).writerows(rows_to_write)

        # персональный CSV в папке графа (без колонки graph)
        per_graph_csv = os.path.join(out_dir, "landmarks.csv")
        with open(per_graph_csv, "w", newline="") as f:
            writer = csv.writer(f)
            writer.writerow(CSV_HEADER[1:])
            for row in rows_to_write:
                writer.writerow(row[1:])

        try:
            plot_landmarks_results(
                name, KS, STRATEGIES,
                basic_mre, sc_mre,
                basic_exact_frac, sc_exact_frac,
                basic_query_times, sc_query_times,
                basic_prep_times, sc_prep_times,
                output_dir=out_dir,
            )
            log(report, f"  Графики сохранены в {out_dir}")
        except Exception as e:
            log(report, f"  Ошибка при построении графиков: {e}")

def run() -> None:
    all_files = collect_all_files(DATASET_ROOT)
    aggregated_csv = os.path.join(OUTPUT_DIR, "landmarks_results.csv")
    with open(aggregated_csv, "w", newline="") as f:
        csv.writer(f).writerow(CSV_HEADER)

    for filepath in all_files:
        process_graph(filepath, aggregated_csv)

    print("\nЭксперименты завершены. Результаты в", OUTPUT_DIR)

if __name__ == "__main__":
    run()
