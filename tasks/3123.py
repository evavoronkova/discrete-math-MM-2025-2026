import math
import heapq

INF = math.inf


class Solution:
    def findAnswer(self, n: int, edges: list[list[int]]) -> list[bool]:
        graph = [[] for _ in range(n)]
        for a, b, w in edges:
            graph[a].append((b, w))
            graph[b].append((a, w))

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

        res = [False] * len(edges)
        start_dists = dijkstra(0)
        shortest_dist = start_dists[n - 1]

        if shortest_dist == INF:
            return res

        end_dists = dijkstra(n - 1)

        for i, (a, b, w) in enumerate(edges):
            if (start_dists[a] + w + end_dists[b] == shortest_dist or
                    start_dists[b] + w + end_dists[a] == shortest_dist):
                res[i] = True
        return res
