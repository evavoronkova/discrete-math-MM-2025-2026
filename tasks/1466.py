from collections import deque


class Solution:
    def minReorder(self, n: int, connections: list[list[int]]) -> int:
        graph = [[] for _ in range(n)]

        for a, b in connections:
            graph[a].append((b, 1))
            graph[b].append((a, 0))

        visited = [0]
        queue = deque()
        queue.append(0)

        min_changed = 0

        while queue:
            current = queue.popleft()
            for neighbour, direction in graph[current]:
                if neighbour not in visited:
                    min_changed += direction
                    visited.append(neighbour)
                    queue.append(neighbour)

        return min_changed
