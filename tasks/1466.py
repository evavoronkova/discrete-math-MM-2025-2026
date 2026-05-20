from collections import deque


class Solution:
    def minReorder(self, n: int, connections: list[list[int]]) -> int:
        # строим граф: узлы - города, а дороги - ребра с весом 1 при направлении от центра, и 0 - к центру
        graph = [[] for _ in range(n)]

        for a, b in connections:
            graph[a].append((b, 1))
            graph[b].append((a, 0))

        visited = [0] * n
        visited[0] = 1
        queue = deque()
        queue.append(0)

        min_changed = 0

        # bfs проходит все дороги от города 0, и считает дороги, которые нужно развернуть к нему
        while queue:
            current = queue.popleft()
            for neighbour, direction in graph[current]:
                if not visited[neighbour]:
                    min_changed += direction
                    visited[neighbour] = 1
                    queue.append(neighbour)

        return min_changed
