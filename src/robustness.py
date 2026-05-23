import random
from collections import defaultdict
from typing import Callable, List, Tuple

from src.analysis import largest_cc_size_ignore

# По заданию граф трактуется как неориентированный (кроме SCC). Поэтому
# для орграфа «степень» — это неориентированная степень (in + out без
# двойного счёта). Кэшируем отсортированный список по id(graph), чтобы
# не пересортировывать 11 раз внутри evaluate_robustness.
_degree_sort_cache: dict = {}


def clear_robustness_cache() -> None:
    """Сбросить кэш отсортированного по степени списка вершин."""
    _degree_sort_cache.clear()


def random_removal(graph, fraction: float) -> set:
    """
    Возвращает множество случайно выбранных вершин
    заданной доли (0..1)
    """
    nodes = list(graph.nodes())
    k = max(0, min(len(nodes), int(len(nodes) * fraction)))
    return set(random.sample(nodes, k))


def degree_based_removal(graph, fraction: float) -> set:
    """
    Возвращает множество вершин с наибольшей степенью заданной доли.
    Для орграфа степень считается неориентированной (in + out unique
    neighbors), как требует задание.
    """
    nodes = list(graph.nodes())
    n = len(nodes)
    k = max(0, min(n, int(n * fraction)))
    if k == 0:
        return set()

    cached_sort = _degree_sort_cache.get(id(graph))
    if cached_sort is None:
        if graph.directed:
            # строим неориентированную смежность и считаем по ней
            weak_adj: dict = defaultdict(set)
            for u, out_set in graph.adj.items():
                for v in out_set:
                    weak_adj[u].add(v)
                    weak_adj[v].add(u)
            degree_of = lambda v: len(weak_adj.get(v, ()))
        else:
            degree_of = graph.degree

        cached_sort = sorted(nodes, key=lambda v: (degree_of(v), v), reverse=True)
        _degree_sort_cache[id(graph)] = cached_sort

    return set(cached_sort[:k])


def evaluate_robustness(
    graph, percentages: List[int], strategy: Callable
) -> List[Tuple[int, float]]:
    """
    Для каждого процента из списка percentages
    вычисляет долю вершин в максимальной компоненте после удаления.
    Возвращает список кортежей (процент, доля_в_МК).
    """
    results = []
    for p in percentages:
        fraction = p / 100.0
        removed = strategy(graph, fraction)
        _, lcc_frac = largest_cc_size_ignore(graph, removed)
        results.append((p, lcc_frac))
    return results
