# расчет всех метрик (1 часть задания)
import os

from experiments._io import graph_output_dir, log, open_report
from experiments.plot_results import plot_degree_distribution
from src.analysis import (
    average_clustering_coefficient,
    connected_components,
    count_triangles,
    degree_distribution,
    degree_stats,
    double_sweep_diameter,
    global_clustering_coefficient,
    largest_cc_size,
    largest_cc_vertices,
    sampled_diameter_and_percentile,
    scc_count_and_largest,
    snowball_diameter_percentile,
)
from src.graph import Graph

DATASET_ROOT = "datasets"
OUTPUT_DIR = "results"
SAMPLE_SIZE = 500

# определение directed из пути файла
def load_graph(filepath: str) -> Graph:
    norm = os.path.normpath(filepath)
    directed = "directed" in norm.split(os.sep)
    return Graph.from_file(filepath, directed=directed)

def analyze_one(filepath: str) -> None:
    name = os.path.relpath(filepath, DATASET_ROOT).replace("/", "_").replace(".", "_")
    out_dir = graph_output_dir(OUTPUT_DIR, name)
    report_path = os.path.join(out_dir, "analysis.txt")

    with open_report(report_path) as report:
        log(report, f"\n===== {name} =====")
        g = load_graph(filepath)

        # базовые характеристики
        n = g.number_of_nodes()
        m = g.number_of_edges()
        max_edges = n * (n - 1) // 2
        dens = m / max_edges if max_edges > 0 else 0.0
        comps = connected_components(g)
        num_comps = len(comps)
        lcc_size, lcc_frac = largest_cc_size(g)

        log(report, f"  Вершин: {n}, Рёбер: {m}")
        log(report, f"  Плотность: {dens:.6f}")
        log(report, f"  Компонент слабой связности: {num_comps}")
        log(report, f"  Доля вершин в макс. компоненте: {lcc_frac:.4f}")

        if g.directed:
            scc_num, scc_frac = scc_count_and_largest(g)
            if scc_num is not None:
                log(report, f"  Компонент сильной связности: {scc_num}")
                log(report, f"  Доля вершин в наибольшей SCC: {scc_frac:.4f}")

        # оценка диаметра и 90-го процентиля
        lcc_verts = largest_cc_vertices(g)
        dbl_diam, dbl_perc = double_sweep_diameter(g, lcc_verts, percentile=90)
        smp_diam, smp_perc = sampled_diameter_and_percentile(
            g, lcc_verts, sample_size=SAMPLE_SIZE, percentile=90
        )
        snb_diam, snb_perc = snowball_diameter_percentile(
            g, lcc_verts, target_size=SAMPLE_SIZE, percentile=90
        )

        log(report, "  Диаметр / 90-й процентиль:")
        log(report, f"    Double sweep:      diam={dbl_diam}, perc90={dbl_perc}")
        log(report, f"    Случайные пары:    diam={smp_diam}, perc90={smp_perc}")
        log(report, f"    Snowball sample:   diam={snb_diam}, perc90={snb_perc}")

        # треугольники и кластерные коэффициенты
        tri = count_triangles(g)
        avg_cl = average_clustering_coefficient(g)
        gcc = global_clustering_coefficient(g)
        log(report, f"  Треугольников: {tri}")
        log(report, f"  Средний кластерный коэффициент: {avg_cl:.6f}")
        log(report, f"  Глобальный кластерный коэффициент: {gcc:.6f}")

        # степени и распределение
        min_d, max_d, avg_d = degree_stats(g)
        log(report, "  Степени:")
        log(report, f"  Минимальная: {min_d}")
        log(report, f"  Максимальная: {max_d}")
        log(report, f"  Средняя: {avg_d:.2f}")

        # график распределения степеней в персональную папку графа
        dist = degree_distribution(g)
        plot_degree_distribution(dist, name, output_dir=out_dir)
        log(report, f"  Графики и отчёт сохранены в {out_dir}")

def run() -> None:
    all_files = []
    for dirpath, _, filenames in os.walk(DATASET_ROOT):
        for fname in filenames:
            all_files.append(os.path.join(dirpath, fname))

    for filepath in all_files:
        analyze_one(filepath)

if __name__ == "__main__":
    run()
