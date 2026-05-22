from __future__ import annotations
 
import random
from array import array
from collections import deque
from heapq import nlargest
from typing import List, Tuple
 
from src.graph import Graph
 
# sentinel «вершина не посещена» для bytearray-расстояний (0..254 — валидные глубины)
_UNREACHABLE = 255
 
class _GraphIndex:
    __slots__ = ("nodes", "node_to_idx", "idx_to_node",
                 "neighbor_offsets", "neighbor_adj", "degrees", "n")
 
    def __init__(self, graph: Graph):
        self.nodes = list(graph.nodes())
        self.n = len(self.nodes)
        self.node_to_idx = {node: i for i, node in enumerate(self.nodes)}
        self.idx_to_node = self.nodes
 
        offsets = array("i", [0]) * (self.n + 1)
        adj = array("i")
        for u_idx, u in enumerate(self.nodes):
            for v in graph.neighbors(u):
                adj.append(self.node_to_idx[v])
            offsets[u_idx + 1] = len(adj)
        self.neighbor_offsets = offsets
        self.neighbor_adj = adj
        self.degrees = [offsets[i + 1] - offsets[i] for i in range(self.n)]
        # после построения CSR оригинальный граф больше не нужен; ссылка
        # на него не сохраняется, чтобы GC мог освободить адъяценции,
        # если вызывающий код тоже отпустит свою ссылку (del graph)
 
    def bfs(self, source_idx: int, need_parent: bool = False, need_farthest: bool = False):
        dist = bytearray([_UNREACHABLE]) * self.n
        parent = array("i", [-1]) * self.n if need_parent else None
 
        offsets = self.neighbor_offsets
        adj = self.neighbor_adj
 
        q = deque([source_idx])
        dist[source_idx] = 0
        farthest = source_idx
 
        while q:
            u = q.popleft()
            du = dist[u]
            if need_farthest and du > dist[farthest]:
                farthest = u
 
            start = offsets[u]
            end = offsets[u + 1]
            for i in range(start, end):
                v = adj[i]
                if dist[v] == _UNREACHABLE:
                    dist[v] = du + 1
                    if parent is not None:
                        parent[v] = u
                    q.append(v)
 
        if need_parent and need_farthest:
            return dist, parent, farthest
        if need_parent:
            return dist, parent
        if need_farthest:
            return dist, farthest
        return dist
 
class LandmarksBasic:
    """
    Базовая оценка расстояний методом ориентиров.
    """
 
    __slots__ = ("_index", "landmarks", "landmark_indices", "dist")
 
    def __init__(self, graph: Graph, landmarks: List[int], _index: _GraphIndex | None = None):
        self._index = _index if _index is not None else _GraphIndex(graph)
        self.landmarks = landmarks
        self.landmark_indices = [self._index.node_to_idx[lm] for lm in landmarks]
 
        self.dist = []
        for lm_idx in self.landmark_indices:
            self.dist.append(self._index.bfs(lm_idx))
 
    @classmethod
    def from_strategy(cls, graph: Graph, k: int, strategy: str = "random", **kwargs):
        """
        Создаёт экземпляр с автоматическим выбором k ориентиров.
        strategy: 'random', 'degree', 'coverage'.
        kwargs: доп параметр (M для coverage).
        """
        index = _GraphIndex(graph)
        seed = kwargs.get("seed", 42)
 
        if strategy == "random":
            landmarks = select_random_landmarks(index, k, seed=seed)
        elif strategy == "degree":
            landmarks = select_degree_landmarks(index, k)
        elif strategy == "coverage":
            M = kwargs.get("M", 500)
            landmarks = select_best_coverage_landmarks(index, k, M=M, seed=seed)
        else:
            raise ValueError(f"Unknown strategy: {strategy}")
 
        return cls(graph, landmarks, _index=index)
 
    def estimate(self, u: int, v: int) -> float:
        iu = self._index.node_to_idx.get(u)
        iv = self._index.node_to_idx.get(v)
        if iu is None or iv is None:
            return -1.0
        if iu == iv:
            return 0.0
 
        best = 10**18
        for dist in self.dist:
            du = dist[iu]
            dv = dist[iv]
            if du != _UNREACHABLE and dv != _UNREACHABLE:
                cand = du + dv
                if cand < best:
                    best = cand
 
        return float(best) if best != 10**18 else -1.0
 
    def estimate_batch(self, pairs: List[Tuple[int, int]]) -> List[float]:
        return [self.estimate(u, v) for u, v in pairs]
 
class ShortestPathTree:
    """
    Дерево кратчайших путей от одного ориентира.
    """
 
    __slots__ = ("_index", "root", "root_idx", "dist", "parent")
 
    def __init__(self, graph: Graph, root: int, _index: _GraphIndex):
        self._index = _index
        self.root = root
        self.root_idx = _index.node_to_idx[root]
 
        self.dist, self.parent = self._build_tree()
        # глубина в BFS-дереве совпадает с dist, отдельное поле не нужно
 
    def _build_tree(self):
        n = self._index.n
        dist = bytearray([_UNREACHABLE]) * n
        parent = array("i", [-1]) * n
 
        offsets = self._index.neighbor_offsets
        adj = self._index.neighbor_adj
 
        q = deque([self.root_idx])
        dist[self.root_idx] = 0
        parent[self.root_idx] = -1
 
        while q:
            u = q.popleft()
            du = dist[u]
            start = offsets[u]
            end = offsets[u + 1]
            for i in range(start, end):
                v = adj[i]
                if dist[v] == _UNREACHABLE:
                    dist[v] = du + 1
                    parent[v] = u
                    q.append(v)
 
        return dist, parent
 
    def lca(self, u: int, v: int) -> int:
        if u == -1 or v == -1:
            return -1
        dist = self.dist
        parent = self.parent
        depth_u = dist[u]
        depth_v = dist[v]
        while depth_u > depth_v:
            u = parent[u]
            if u == -1:
                return -1
            depth_u -= 1
        while depth_v > depth_u:
            v = parent[v]
            if v == -1:
                return -1
            depth_v -= 1
        while u != v:
            u = parent[u]
            v = parent[v]
            if u == -1 or v == -1:
                return -1
        return u
 
    def distance_sc(self, u: int, v: int) -> float:
        iu = self._index.node_to_idx.get(u)
        iv = self._index.node_to_idx.get(v)
        if iu is None or iv is None:
            return float("inf")
        if iu == iv:
            return 0.0
 
        w = self.lca(iu, iv)
        if w == -1:
            return float("inf")
 
        return float(self.dist[iu] + self.dist[iv] - 2 * self.dist[w])
 
class LandmarksSC:
    """
    Оценка расстояний через SPT и LCA.
    """
 
    __slots__ = ("_index", "landmarks", "landmark_indices", "trees")
 
    def __init__(self, graph: Graph, landmarks: List[int], _index: _GraphIndex | None = None):
        self._index = _index if _index is not None else _GraphIndex(graph)
        self.landmarks = landmarks
        self.landmark_indices = [self._index.node_to_idx[lm] for lm in landmarks]
        self.trees = [ShortestPathTree(graph, lm, self._index) for lm in landmarks]
 
    @classmethod
    def from_strategy(cls, graph: Graph, k: int, strategy: str = "random", **kwargs):
        index = _GraphIndex(graph)
        seed = kwargs.get("seed", 42)
 
        if strategy == "random":
            landmarks = select_random_landmarks(index, k, seed=seed)
        elif strategy == "degree":
            landmarks = select_degree_landmarks(index, k)
        elif strategy == "coverage":
            M = kwargs.get("M", 500)
            landmarks = select_best_coverage_landmarks(index, k, M=M, seed=seed)
        else:
            raise ValueError(f"Unknown strategy: {strategy}")
 
        return cls(graph, landmarks, _index=index)
 
    def estimate(self, u: int, v: int) -> float:
        best = float("inf")
        for tree in self.trees:
            d = tree.distance_sc(u, v)
            if d < best:
                best = d
        return best if best != float("inf") else -1.0
 
def select_random_landmarks(index: _GraphIndex, count: int, seed: int = 42) -> List[int]:
    rng = random.Random(seed)
    if count >= index.n:
        return index.nodes[:]
    idxs = rng.sample(range(index.n), count)
    return [index.idx_to_node[i] for i in idxs]
 
def select_degree_landmarks(index: _GraphIndex, count: int) -> List[int]:
    best = nlargest(count, range(index.n), key=lambda i: (index.degrees[i], -i))
    return [index.idx_to_node[i] for i in best]
 
def select_best_coverage_landmarks(index: _GraphIndex, k: int, M: int = 500, seed: int = 42) -> List[int]:
    rng = random.Random(seed)
    if k >= index.n:
        return index.nodes[:]
 
    counts = array("i", [0]) * index.n
    sources = rng.sample(range(index.n), min(M, index.n))
 
    for s in sources:
        _, parent, farthest = index.bfs(s, need_parent=True, need_farthest=True)
        cur = farthest
        while cur != -1:
            counts[cur] += 1
            if cur == s:
                break
            cur = parent[cur]
 
    best = nlargest(k, range(index.n), key=lambda i: (counts[i], -i))
    return [index.idx_to_node[i] for i in best]
