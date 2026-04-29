from collections import defaultdict
import math
import heapq

INF = math.inf


class Solution:
    def findAnswer(self, n: int, edges: list[list[int]]) -> list[bool]:
        graph = defaultdict(list)
        for a, b, w in edges:
            graph[a].append((b, w))
            graph[b].append((a, w))

        def dijkstra(start_node: int) -> list:
            dists = [INF] * n
            dists[start_node] = 0
            queue = [(0, start_node)]

            while queue:
                dist, node = heapq.heappop(queue)
                if dist <= dists[node]:
                    for neighbour, w in graph[node]:
                        if dist + w < dists[neighbour]:
                            dists[neighbour] = dist + w
                            heapq.heappush(queue, (dists[neighbour], neighbour))
            return dists

        start_dists = dijkstra(0)
        end_dists = dijkstra(n - 1)
        shortest_dist = start_dists[n - 1]
        res = [False] * len(edges)

        if shortest_dist == INF:
            return res

        for i, (a, b, w) in enumerate(edges):
            if (start_dists[a] + w + end_dists[b] == shortest_dist or
                    start_dists[b] + w + end_dists[a] == shortest_dist):
                res[i] = True
        return res
