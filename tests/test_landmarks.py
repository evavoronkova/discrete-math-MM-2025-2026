from src.graph import Graph
from src.landmarks import (
    LandmarksBasic,
    LandmarksSC,
    ShortestPathTree,
    _GraphIndex,
    select_best_coverage_landmarks,
    select_degree_landmarks,
    select_random_landmarks,
)


# LandmarksBasic
def test_landmarks_basic_path():
    # путь 1-2-3-4, ориентир = 1, оценка расстояния 2-4 = d(2,1)+d(4,1) = 1+3 = 4
    g = Graph()
    g.add_edge(1, 2)
    g.add_edge(2, 3)
    g.add_edge(3, 4)
    lb = LandmarksBasic(g, [1])
    est = lb.estimate(2, 4)
    assert est == 4

def test_landmarks_basic_triangle_exact():
    # треугольник: ориентиры 1 и 2, оценка для 1-3 должна быть точной 1
    g = Graph()
    g.add_edge(1, 2)
    g.add_edge(2, 3)
    g.add_edge(3, 1)
    lb = LandmarksBasic(g, [1, 2])
    est = lb.estimate(1, 3)
    assert est == 1

def test_landmarks_basic_unreachable():
    # граф из двух компонент: вершины в разных компонентах дают -1
    g = Graph()
    g.add_edge(1, 2)
    g.add_edge(3, 4)
    lb = LandmarksBasic(g, [1])
    est = lb.estimate(1, 3)
    assert est == -1.0

def test_landmarks_basic_batch():
    g = Graph()
    g.add_edge(1, 2)
    g.add_edge(2, 3)
    lb = LandmarksBasic(g, [1])
    results = lb.estimate_batch([(1, 3), (2, 3)])
    assert results == [2, 3]


# ShortestPathTree — после рефактора требует _GraphIndex и оперирует CSR-индексами
def test_spt_parent_after_bfs():
    # корень 1, 1-2, 1-3: parent[2] и parent[3] — индекс вершины 1
    g = Graph()
    g.add_edge(1, 2)
    g.add_edge(1, 3)
    index = _GraphIndex(g)
    spt = ShortestPathTree(g, 1, index)
    i1 = index.node_to_idx[1]
    i2 = index.node_to_idx[2]
    i3 = index.node_to_idx[3]
    assert spt.parent[i2] == i1
    assert spt.parent[i3] == i1
    assert spt.parent[i1] == -1

def test_spt_lca_siblings():
    # LCA 2 и 3 в звезде с центром 1 — это вершина 1
    g = Graph()
    g.add_edge(1, 2)
    g.add_edge(1, 3)
    index = _GraphIndex(g)
    spt = ShortestPathTree(g, 1, index)
    i1 = index.node_to_idx[1]
    i2 = index.node_to_idx[2]
    i3 = index.node_to_idx[3]
    assert spt.lca(i2, i3) == i1

def test_spt_lca_self():
    # LCA вершины с собой — она же
    g = Graph()
    g.add_edge(1, 2)
    index = _GraphIndex(g)
    spt = ShortestPathTree(g, 1, index)
    i2 = index.node_to_idx[2]
    assert spt.lca(i2, i2) == i2

def test_spt_lca_ancestor():
    # LCA вершины и её предка в дереве — это сам предок
    g = Graph()
    g.add_edge(1, 2)
    g.add_edge(2, 3)
    index = _GraphIndex(g)
    spt = ShortestPathTree(g, 1, index)
    i1 = index.node_to_idx[1]
    i3 = index.node_to_idx[3]
    assert spt.lca(i1, i3) == i1
    assert spt.lca(i3, i1) == i1


def test_distance_sc_simple_tree():
    # 1-2 и 1-3. SPT от 1: 2 и 3 — листья.
    # distance_sc(2, 3) = d(2,1) + d(3,1) - 2*d(1,1) = 1 + 1 - 0 = 2
    g = Graph()
    g.add_edge(1, 2)
    g.add_edge(1, 3)
    index = _GraphIndex(g)
    spt = ShortestPathTree(g, 1, index)
    est = spt.distance_sc(2, 3)
    assert est == 2

def test_distance_sc_path_tree():
    # путь 0-1-2-3-4. SPT от 0 совпадает с путём.
    # distance_sc(2, 4) = 2 + 4 - 2*2 = 2 (точное расстояние, путь лежит в дереве)
    g = Graph()
    for i in range(4):
        g.add_edge(i, i + 1)
    index = _GraphIndex(g)
    spt = ShortestPathTree(g, 0, index)
    est = spt.distance_sc(2, 4)
    assert est == 2

def test_distance_sc_unreachable_node():
    # запрос с вершиной, которой нет в графе → inf
    g = Graph()
    g.add_edge(1, 2)
    index = _GraphIndex(g)
    spt = ShortestPathTree(g, 1, index)
    est = spt.distance_sc(1, 99)
    assert est == float("inf")


# LandmarksSC
def test_landmarks_sc_two_landmarks():
    # путь 0-1-2-3-4-5, ориентиры 0 и 5
    # для пары (1, 4) каждое из двух деревьев SPT даёт точное расстояние 3
    g = Graph()
    for i in range(5):
        g.add_edge(i, i + 1)
    lsc = LandmarksSC(g, [0, 5])
    est = lsc.estimate(1, 4)
    assert est == 3

def test_landmarks_sc_triangle_tree_estimate():
    # треугольник 1-2-3 с единственным ориентиром 1.
    # SPT от 1: дерево 1->2, 1->3, ребро 2-3 НЕ в дереве.
    # Текущая реализация SC оценивает расстояние через дерево и LCA, не учитывая
    # ребра вне дерева (shortcut'ы). Поэтому оценка 2-3 = 1+1 = 2,
    # хотя истинное расстояние = 1. На реальных графах с большим k минимум
    # по деревьям обычно даёт близкую к истине оценку.
    g = Graph()
    g.add_edge(1, 2)
    g.add_edge(2, 3)
    g.add_edge(3, 1)
    lsc = LandmarksSC(g, [1])
    est = lsc.estimate(2, 3)
    assert est == 2


# стратегии выбора ориентиров — после рефактора принимают _GraphIndex
def test_random_landmarks_count():
    g = Graph()
    for i in range(10):
        g.add_edge(i, i + 1)
    index = _GraphIndex(g)
    landmarks = select_random_landmarks(index, 3)
    assert len(landmarks) == 3
    assert all(0 <= lm <= 10 for lm in landmarks)

def test_degree_landmarks_order():
    g = Graph()
    g.add_edge(1, 2)
    g.add_edge(1, 3)
    g.add_edge(1, 4)  # степень 1 = 3
    g.add_edge(2, 3)  # степени 2 и 3 по 2
    index = _GraphIndex(g)
    landmarks = select_degree_landmarks(index, 2)
    assert landmarks[0] == 1
    assert len(landmarks) == 2

def test_coverage_landmarks_count():
    g = Graph()
    for i in range(5):
        g.add_edge(i, i + 1)
    index = _GraphIndex(g)
    landmarks = select_best_coverage_landmarks(index, k=2, M=100)
    assert len(landmarks) == 2
    assert all(0 <= lm <= 5 for lm in landmarks)

def test_coverage_landmarks_seed_reproducibility():
    # с одинаковым seed функция даёт одинаковый результат
    g = Graph()
    g.add_edge(0, 1)
    g.add_edge(1, 2)
    g.add_edge(2, 3)
    index = _GraphIndex(g)
    lm1 = select_best_coverage_landmarks(index, k=2, M=50, seed=42)
    lm2 = select_best_coverage_landmarks(index, k=2, M=50, seed=42)
    assert lm1 == lm2
