from collections import deque
import random

class LandmarkEstimator:
    def __init__(self, graph):
        self.graph = graph
        self.landmarks = []
        self.distances = {}     # {landmark: {node: dist}}
        self.spt_parents = {}   # {landmark: {node: parent}}
        self.spt_depths = {}    # {landmark: {node: depth}}

    def select_landmarks(self, num_landmarks=20, method='degree'):
        """Пункт 2. Стратегии выбора ориентиров: 'random' или 'degree'"""
        if method == 'random':
            self.landmarks = random.sample(list(self.graph.keys()), min(num_landmarks, len(self.graph)))
        elif method == 'degree':
            sorted_nodes = sorted(self.graph.keys(), key=lambda n: len(self.graph[n]), reverse=True)
            self.landmarks = sorted_nodes[:num_landmarks]

        for l in self.landmarks:
            self._build_spt(l)

    def _build_spt(self, landmark):
        """Построение Shortest Path Tree (SPT) через BFS"""
        dists = {landmark: 0}
        parents = {landmark: None}
        depths = {landmark: 0}
        queue = deque([landmark])

        while queue:
            u = queue.popleft()
            for v in self.graph.get(u, set()):
                if v not in dists:
                    dists[v] = dists[u] + 1
                    parents[v] = u
                    depths[v] = depths[u] + 1
                    queue.append(v)
        self.distances[landmark] = dists
        self.spt_parents[landmark] = parents
        self.spt_depths[landmark] = depths

    def estimate_basic(self, u, v):
        """Алгоритм Landmarks-Basic (Верхняя оценка по неравенству треугольника)"""
        min_dist = float('inf')
        for l in self.landmarks:
            dists = self.distances[l]
            if u in dists and v in dists:
                min_dist = min(min_dist, dists[u] + dists[v])
        return min_dist if min_dist != float('inf') else -1

    def _find_lca(self, l, u, v):
        """Поиск Наименьшего общего предка (LCA) в дереве SPT ориентира l"""
        parents = self.spt_parents[l]
        depths = self.spt_depths[l]
        if u not in depths or v not in depths: return None

        curr_u, curr_v = u, v
        while depths[curr_u] > depths[curr_v]:
            curr_u = parents[curr_u]
        while depths[curr_v] > depths[curr_u]:
            curr_v = parents[curr_v]
        while curr_u != curr_v:
            curr_u = parents[curr_u]
            curr_v = parents[curr_v]
        return curr_u

    def estimate_lca(self, u, v):
        """
        Модификация Landmarks-LCA.
        Формула: d(u,v) = depth(u) + depth(v) - 2 * depth(LCA(u,v))
        """
        min_dist = float('inf')
        for l in self.landmarks:
            depths = self.spt_depths[l]
            if u in depths and v in depths:
                lca = self._find_lca(l, u, v)
                if lca is not None:
                    dist = depths[u] + depths[v] - 2 * depths[lca]
                    min_dist = min(min_dist, dist)
        return min_dist if min_dist != float('inf') else -1
