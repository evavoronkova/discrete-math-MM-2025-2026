import sys
import time
import random
import numpy as np

sys.path.insert(0, '.')

from src.graph_utils import load_graph, get_weakly_connected_components, bfs_distances
from src.landmarks import LandmarkEstimator


def run_benchmark(graph_data, graph, all_nodes, is_directed, sample_size=200):
    print(f"\n=== ЗАПУСК БЕНЧМАРКА (Выборка: {sample_size} пар) ===")

    # 1. Готовим выборку пар и их ТОЧНЫЕ расстояния (Ground Truth)
    print("[1/3] Вычисление точных расстояний для выборки (это может занять время)...")
    wcc = get_weakly_connected_components(graph, graph_data[1] if is_directed else None, all_nodes)
    lcc = max(wcc, key=len)
    nodes_list = list(lcc)

    pairs = random.sample(nodes_list, min(sample_size, len(nodes_list)))
    exact_distances = []

    for u in pairs:
        dists = bfs_distances(graph, u)
        for v in random.sample(nodes_list, min(5, len(nodes_list))):  # Берем 5 случайных соседей для каждой вершины
            if u != v and v in dists:
                exact_distances.append((u, v, dists[v]))

    print(f"Подготовлено {len(exact_distances)} пар с известными расстояниями.")

    # 2. Конфигурации теста
    k_values = [10, 30, 50]  # Количество лендмарков
    methods = ['random', 'degree']  # Способы выбора

    results = []

    for method in methods:
        for k in k_values:
            print(f"\n[2/3] Тест: Метод={method}, k={k}")

            estimator = LandmarkEstimator(graph)

            # Замеряем время препроцессинга (построение деревьев/таблиц)
            start_prep = time.time()
            estimator.select_landmarks(num_landmarks=k, method=method)
            prep_time = time.time() - start_prep
            print(f"      Препроцессинг: {prep_time:.2f} сек.")

            errors_basic = []
            errors_lca = []
            underestimates_basic = 0
            underestimates_lca = 0

            # Замеряем время запросов
            start_query_basic = time.time()
            for u, v, exact_d in exact_distances:
                est_b = estimator.estimate_basic(u, v)
                if est_b != -1:
                    errors_basic.append(abs(est_b - exact_d))
                    if est_b < exact_d: underestimates_basic += 1
            query_time_basic = (time.time() - start_query_basic) / len(exact_distances)

            start_query_lca = time.time()
            for u, v, exact_d in exact_distances:
                est_l = estimator.estimate_lca(u, v)
                if est_l != -1:
                    errors_lca.append(abs(est_l - exact_d))
                    if est_l < exact_d: underestimates_lca += 1
            query_time_lca = (time.time() - start_query_lca) / len(exact_distances)

            # Считаем метрики
            mae_basic = np.mean(errors_basic) if errors_basic else 0
            mae_lca = np.mean(errors_lca) if errors_lca else 0
            exact_match_basic = sum(1 for e in errors_basic if e == 0) / len(errors_basic) if errors_basic else 0
            exact_match_lca = sum(1 for e in errors_lca if e == 0) / len(errors_lca) if errors_lca else 0

            results.append({
                'Method': method, 'K': k, 'Prep Time': prep_time,
                'Query Basic (ms)': query_time_basic * 1000, 'MAE Basic': mae_basic, 'Exact Basic': exact_match_basic,
                'Under Basic': underestimates_basic,
                'Query LCA (ms)': query_time_lca * 1000, 'MAE LCA': mae_lca, 'Exact LCA': exact_match_lca,
                'Under LCA': underestimates_lca
            })

    # 3. Печатаем таблицу
    print("\n[3/3] РЕЗУЛЬТАТЫ ИССЛЕДОВАНИЯ:")
    print("-" * 150)
    print(
        f"{'Метод':<10} | {'K':<5} | {'Время пре-проц.':<15} | {'Запрос Basic(мс)':<18} | {'MAE Basic':<10} | {'Точн. Basic':<12} | {'Запрос LCA(мс)':<15} | {'MAE LCA':<9} | {'Точн. LCA':<11} | {'Недооц. LCA':<12}")
    print("-" * 150)
    for r in results:
        print(
            f"{r['Method']:<10} | {r['K']:<5} | {r['Prep Time']:<15.2f} | {r['Query Basic (ms)']:<18.4f} | {r['MAE Basic']:<10.3f} | {r['Exact Basic']:<12.1%} | {r['Query LCA (ms)']:<15.4f} | {r['MAE LCA']:<9.3f} | {r['Exact LCA']:<11.1%} | {r['Under LCA']:<12}")
    print("-" * 150)


if __name__ == "__main__":
    path = input("Введите путь к файлу датасета (или 'test'): ").strip()
    is_dir = input("Ориентированный? (y/n): ").strip().lower() == 'y'

    if path == 'test':
        graph = {i: set(random.sample(range(150), random.randint(2, 5))) for i in range(150)}
        for u in list(graph.keys()):
            for v in graph[u]:
                if v not in graph: graph[v] = set()
                graph[v].add(u)
        all_nodes = set(graph.keys())
        run_benchmark(None, graph, all_nodes, is_dir, sample_size=50)
    else:
        graph, rev_graph, all_nodes = load_graph(path, delimiter=None, is_directed=is_dir)
        run_benchmark((graph, rev_graph, all_nodes), graph, all_nodes, is_dir, sample_size=200)
