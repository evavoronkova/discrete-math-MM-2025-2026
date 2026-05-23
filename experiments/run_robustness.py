# исследование устойчивости для всех графов
import csv
import os

from experiments._io import graph_output_dir, log, open_report
from experiments.plot_results import plot_robustness
from src.graph import Graph
from src.robustness import degree_based_removal, evaluate_robustness, random_removal

DATASET_ROOT = "datasets"
OUTPUT_DIR = "results"
PERCENTAGES = list(range(0, 101, 10))
MAX_NODES_FOR_ROBUSTNESS = 500_000

# определение directed из пути файла
def load_graph(filepath: str) -> Graph:
    norm = os.path.normpath(filepath)
    directed = "directed" in norm.split(os.sep)
    return Graph.from_file(filepath, directed=directed)

def write_robustness_csv(path: str, percentages, rand_res, deg_res) -> None:
    with open(path, "w", newline="") as f:
        writer = csv.writer(f)
        writer.writerow(["percent_removed", "random_lcc_frac", "degree_lcc_frac"])
        for i, p in enumerate(percentages):
            writer.writerow([p, rand_res[i][1], deg_res[i][1]])

def analyze_one(filepath: str) -> None:
    name = os.path.relpath(filepath, DATASET_ROOT).replace("/", "_").replace(".", "_")
    out_dir = graph_output_dir(OUTPUT_DIR, name)
    report_path = os.path.join(out_dir, "robustness.txt")

    with open_report(report_path) as report:
        log(report, f"\n===== {name} =====")
        g = load_graph(filepath)
        n = g.number_of_nodes()

        # если граф слишком большой, пока пропускаем устойчивость
        if n > MAX_NODES_FOR_ROBUSTNESS:
            log(report, f"  Граф слишком большой ({n} вершин), устойчивость не вычисляется")
            return

        # случайное удаление
        log(report, "  Случайное удаление...")
        rand_res = evaluate_robustness(g, PERCENTAGES, random_removal)
        # удаление по степени
        log(report, "  Удаление по наибольшей степени...")
        deg_res = evaluate_robustness(g, PERCENTAGES, degree_based_removal)

        log(report, "  % удалённых | случайное | по степени")
        for i, p in enumerate(PERCENTAGES):
            log(report, f"    {p:>3d}        | {rand_res[i][1]:.4f}    | {deg_res[i][1]:.4f}")

        # сохраняем сырые данные и график
        csv_path = os.path.join(out_dir, "robustness.csv")
        write_robustness_csv(csv_path, PERCENTAGES, rand_res, deg_res)
        plot_robustness(rand_res, deg_res, name=name, output_dir=out_dir)
        log(report, f"  Данные и график сохранены в {out_dir}")

def run() -> None:
    all_files = []
    for dirpath, _, filenames in os.walk(DATASET_ROOT):
        for fname in filenames:
            all_files.append(os.path.join(dirpath, fname))

    for filepath in all_files:
        analyze_one(filepath)

if __name__ == "__main__":
    run()
