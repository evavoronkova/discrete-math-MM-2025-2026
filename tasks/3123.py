import math
import heapq

INF = math.inf


class Solution:
    def findAnswer(self, n: int, edges: list[list[int]]) -> list[bool]:
        graph = [[] for _ in range(n)]  # делаем граф с вершинами и с ребрами с весом
        for a, b, w in edges:
            graph[a].append((b, w))
            graph[b].append((a, w))

        # используем алгоритм Дейкстры для поиска кратчайшего пути
        def dijkstra(start_node: int) -> list:
            dists = [INF] * n
            dists[start_node] = 0
            queue = [(0, start_node)]

            while queue:
                dist, node = heapq.heappop(queue)
                if dist > dists[node]:
                    continue
                for neighbour, w in graph[node]:
                    if dist + w < dists[neighbour]:
                        dists[neighbour] = dist + w
                        heapq.heappush(queue, (dists[neighbour], neighbour))
            return dists

        res = [False] * len(edges)  # массив с boolean ребер являющихся частью кратчайшего пути
        start_dists = dijkstra(0)  # запускаем Дейкстру с начала
        shortest_dist = start_dists[n - 1]

        if shortest_dist == INF:
            return res

        end_dists = dijkstra(n - 1)  # запускаем Дейкстру с конца

        # ребро делаем проверки на кратчайший путь для всех ребер - путь от начала до ребра и от ребра до конца
        for i, (a, b, w) in enumerate(edges):
            if (start_dists[a] + w + end_dists[b] == shortest_dist or
                    start_dists[b] + w + end_dists[a] == shortest_dist):
                res[i] = True
        return res
