from collections import deque

class Solution:
    def findShortestCycle(self, n: int, edges: list[list[int]]) -> int:
        graph = [[] for _ in range(n)]
        for a, b in edges:
            graph[a].append(b)
            graph[b].append(a)

        INF = 10 ** 9
        answer = INF

        # перебираем каждую вершину как стартовую для bfs
        for start in range(n):
            # треугольник - минимальный цикл
            if answer == 3:
                return 3

            dist = [-1] * n
            parent = [-1] * n
            queue = deque([start])
            dist[start] = 0

            while queue:
                u = queue.popleft()
                for v in graph[u]:
                    if dist[v] == -1:  # ещё не посещён
                        dist[v] = dist[u] + 1
                        parent[v] = u
                        queue.append(v)
                    elif v != parent[u]:  # сосед не родитель -> это новый путь
                        # до уже посещённой вершины -> нашли цикл
                        cycle_len = dist[u] + dist[v] + 1
                        if cycle_len < answer:
                            answer = cycle_len

        return answer if answer != INF else -1
