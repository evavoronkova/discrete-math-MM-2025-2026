import os
import random
import time
from src.graph_utils import load_graph, get_weakly_connected_components, get_strongly_connected_components, \
    bfs_distances
from src.metrics import *
from src.landmarks import LandmarkEstimator


def run_full_analysis(graph, rev_graph, all_nodes, is_directed):
    print("\n[Вычисление базовых характеристик...]")
    # Передаем all_nodes
    V, E, density = calc_basic_stats(graph, is_directed, all_nodes)
    print(f"Вершин (V): {V}, Рёбер (E): {E}, Плотность: {density:.6f}")

    # Передаем rev_graph и all_nodes для правильного поиска WCC в орграфе
    wcc = get_weakly_connected_components(graph, rev_graph, all_nodes)
    max_wcc = max(wcc, key=len) if wcc else set()
    print(f"Компонент слабой связности: {len(wcc)}")
    print(f"Доля вершин в крупнейшей WCC: {len(max_wcc) / V:.2%}") # Теперь будет < 100%

    if is_directed and rev_graph:
        scc = get_strongly_connected_components(graph, rev_graph, all_nodes)
        max_scc_len = max(len(c) for c in scc) if scc else 0
        print(f"Компонент сильной связности: {len(scc)}")
        print(f"Доля вершин в крупнейшей SCC: {max_scc_len / V:.2%}")

    print("\n[Оценка диаметра и процентилей для крупнейшей компоненты...]")
    print(f"Диаметр (Double Sweep): {estimate_diameter_double_sweep(graph, max_wcc)}")
    print(f"90-й процентиль (Random): {estimate_percentile_random(graph, max_wcc, 300)}")
    print(f"90-й процентиль (Snowball): {estimate_percentile_snowball(graph, max_wcc, 300, 300)}")

    print("\n[Вычисление кластеризации и треугольников...]")
    g_cc, a_cc, triangles = calc_clustering_and_triangles(graph)
    print(f"Треугольников: {triangles}\nГлобальный CC (по формуле): {g_cc:.5f}\nСредний CC: {a_cc:.5f}")

    os.makedirs("plots", exist_ok=True)
    min_d, max_d, avg_d = calc_degree_stats(graph, save_plot_path="plots/degree_distribution.png")
    print(f"Степени: Min={min_d}, Max={max_d}, Avg={avg_d:.2f} (График сохранен в plots/)")

    print("\n[Симуляция удаления х% узлов (Пункт 1.В)]")
    for x in [10, 30]:
        sh_r, sh_h = simulate_network_attack(graph, x, all_nodes)
        print(f" Удаление {x}%: Случайное = {sh_r:.2%}, Хабы = {sh_h:.2%}")


def main():
    print("=== КЛИЕНТ ДЕМОНСТРАЦИИ ПРОЕКТА: LANDMARKS-LCA ===")
    path = input("Введите путь к файлу датасета (или наберите 'test' для синтетики): ").strip()

    is_directed = input("Граф ориентированный? (y/n): ").strip().lower() == 'y'

    if path.lower() == 'test':
        print("Генерация тестовой сети...")
        graph = {i: set(random.sample(range(150), random.randint(2, 5))) for i in range(150)}
        for u in list(graph.keys()):
            for v in graph[u]:
                if v not in graph: graph[v] = set()
                graph[v].add(u)
        rev_graph = graph if is_directed else None
    else:
        if not os.path.exists(path):
            print("Файл не найден!")
            return
        graph, rev_graph, all_nodes = load_graph(path, delimiter=None, is_directed=is_directed)

    while True:
        print("\n МЕНЮ УПРАВЛЕНИЯ:")
        print("1. Запустить полный структурный анализ (Часть 1)")
        print("2. Оценить расстояние между конкретной парой (Landmarks Basic vs LCA)")
        print("3. Выход")
        choice = input("Выберите действие: ").strip()

        if choice == "1":
            run_full_analysis(graph, rev_graph, all_nodes, is_directed)
        elif choice == "2":
            u = int(input("Введите вершину U: "))
            v = int(input("Введите вершину V: "))

            estimator = LandmarkEstimator(graph)
            estimator.select_landmarks(num_landmarks=15, method='degree')

            exact = bfs_distances(graph, u).get(v, -1)
            est_b = estimator.estimate_basic(u, v)
            est_l = estimator.estimate_lca(u, v)

            print(f"\n[Результаты оценки d({u}, {v})]:")
            print(f" -> Точное расстояние (BFS): {exact}")
            print(f" -> Оценка Landmarks-Basic: {est_b}")
            print(f" -> Оценка Модификации Landmarks-LCA: {est_l}")
        elif choice == "3":
            break


if __name__ == "__main__":
    main()
