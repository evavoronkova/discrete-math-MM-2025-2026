from collections import defaultdict
import numpy as np
import random
import matplotlib.pyplot as plt
from src.graph_utils import bfs_distances, get_weakly_connected_components


def calc_basic_stats(graph, is_directed=False, all_nodes=None):
    # Если передали all_nodes (для орграфов), берем его размер. Иначе - размер словаря.
    V = len(all_nodes) if all_nodes is not None else len(graph)

    total_edges = sum(len(neighbors) for neighbors in graph.values())
    E = total_edges if is_directed else total_edges // 2

    max_edges = V * (V - 1) if is_directed else (V * (V - 1)) / 2
    density = E / max_edges if max_edges > 0 else 0
    return V, E, density


def calc_degree_stats(graph, save_plot_path=None):
    """Пункт 1.А.5. Расчет степеней и построение графиков в обычной и log-log шкалах."""
    degrees = [len(neighbors) for neighbors in graph.values()]
    min_deg = min(degrees) if degrees else 0
    max_deg = max(degrees) if degrees else 0
    avg_deg = np.mean(degrees) if degrees else 0

    if save_plot_path:
        plt.figure(figsize=(12, 5))

        # Обычная шкала
        plt.subplot(1, 2, 1)
        plt.hist(degrees, bins=30, color='skyblue', edgecolor='black')
        plt.title("Degree Distribution")
        plt.xlabel("Degree")
        plt.ylabel("Count")

        # Log-log шкала
        plt.subplot(1, 2, 2)
        degree_counts = defaultdict(int)
        for d in degrees:
            degree_counts[d] += 1
        x = np.array(list(degree_counts.keys()))
        y = np.array(list(degree_counts.values()))

        mask = (x > 0) & (y > 0)
        plt.loglog(x[mask], y[mask], 'o', color='red', markersize=4)
        plt.title("Degree Distribution (Log-Log)")
        plt.xlabel("Degree (log)")
        plt.ylabel("Count (log)")

        plt.tight_layout()
        plt.savefig(save_plot_path)
        plt.close()

    return min_deg, max_deg, avg_deg


def estimate_diameter_double_sweep(graph, component_nodes):
    """Пункт A.2.a - Метод Double Sweep"""
    r = random.choice(list(component_nodes))
    dist_r = bfs_distances(graph, r)
    a = max(dist_r, key=dist_r.get)
    dist_a = bfs_distances(graph, a)
    b = max(dist_a, key=dist_a.get)
    return dist_a[b]


def estimate_percentile_random(graph, component_nodes, sample_size=500):
    """Пункт A.2.b - 90-й процентиль на случайных парах"""
    nodes_list = list(component_nodes)
    distances = []
    for _ in range(min(sample_size, len(nodes_list) * (len(nodes_list) - 1) // 2)):
        u, v = random.sample(nodes_list, 2)
        dist_u = bfs_distances(graph, u)
        if v in dist_u:
            distances.append(dist_u[v])
    return np.percentile(distances, 90) if distances else 0.0


def build_snowball_subgraph(graph, start_nodes, target_size=500):
    """Вспомогательный метод построения подграфа 'Снежный ком'"""
    visited = set(start_nodes)
    queue = list(start_nodes)
    while queue and len(visited) < target_size:
        u = queue.pop(0)
        for v in graph.get(u, set()):
            if v not in visited:
                visited.add(v)
                queue.append(v)
                if len(visited) >= target_size:
                    break
    subgraph = defaultdict(set)
    for u in visited:
        subgraph[u] = graph[u] & visited
    return subgraph


def estimate_percentile_snowball(graph, component_nodes, target_size=500, sample_size=500):
    """Пункт A.2.c - Расстояния по подграфу 'Снежный ком'"""
    start_node = random.choice(list(component_nodes))
    start_neighbors = list(graph[start_node])[:3]
    if not start_neighbors:
        start_neighbors = [start_node]

    snowball_graph = build_snowball_subgraph(graph, start_neighbors, target_size)
    snowball_nodes = list(snowball_graph.keys())
    distances = []

    for _ in range(min(sample_size, len(snowball_nodes) ** 2)):
        if len(snowball_nodes) < 2: break
        u, v = random.sample(snowball_nodes, 2)
        dist_u = bfs_distances(snowball_graph, u)
        if v in dist_u:
            distances.append(dist_u[v])

    return np.percentile(distances, 90) if distances else 0.0


def calc_clustering_and_triangles(graph):
    """
    Пункты A.3 и A.4. Вычисление треугольников,
    а также среднего и глобального кластерных коэффициентов СТРОГО по формуле задания.
    """
    local_cc_sum = 0.0
    local_cc_list = []
    total_triangles = 0
    V = len(graph)

    for u in graph:
        neighbors_u = graph[u]
        k_u = len(neighbors_u)
        if k_u >= 2:
            L_u = sum(len(neighbors_u & graph.get(v, set())) for v in neighbors_u) // 2
            total_triangles += L_u
            C_u = (2 * L_u) / (k_u * (k_u - 1))
            local_cc_sum += C_u
            local_cc_list.append(C_u)
        else:
            # По определению из формулы пункта 4: Cl_u = 0, если степень < 2
            local_cc_sum += 0.0

    actual_triangles = total_triangles // 3
    avg_cc = np.mean(local_cc_list) if local_cc_list else 0.0

    # Вычисляем по формуле из пункта 4: (1 / |V|) * Sum(Cl_u)
    global_cc_formula = local_cc_sum / V if V > 0 else 0.0

    return global_cc_formula, avg_cc, actual_triangles


def simulate_network_attack(graph, x_percent, all_nodes=None):
    """Пункт B. Симуляция уязвимости сети (Случайное удаление vs Удаление хабов)"""
    nodes = list(graph.keys())
    num_to_remove = int(len(nodes) * (x_percent / 100))

    # 1. Случайное удаление
    random_remove = set(random.sample(nodes, num_to_remove))
    graph_random = defaultdict(set)
    for u in graph:
        if u in random_remove: continue
        graph_random[u] = graph[u] - random_remove
    comps_rand = get_weakly_connected_components(graph_random)
    max_rand_len = max([len(c) for c in comps_rand]) if comps_rand else 0
    total_remaining = len(all_nodes) - num_to_remove if all_nodes else len(graph_random)
    share_random = max_rand_len / total_remaining if total_remaining > 0 else 0

    # 2. Удаление хабов
    sorted_by_degree = sorted(graph.keys(), key=lambda n: len(graph[n]), reverse=True)
    hub_remove = set(sorted_by_degree[:num_to_remove])
    graph_hubs = defaultdict(set)
    for u in graph:
        if u in hub_remove: continue
        graph_hubs[u] = graph[u] - hub_remove
    comps_hubs = get_weakly_connected_components(graph_hubs)
    max_hubs_len = max([len(c) for c in comps_hubs]) if comps_hubs else 0
    total_remaining_hubs = len(all_nodes) - num_to_remove if all_nodes else len(graph_hubs)
    share_hubs = max_hubs_len / total_remaining_hubs if total_remaining_hubs > 0 else 0

    return share_random, share_hubs
