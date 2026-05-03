from collections import deque

class Solution:
    def possibleBipartition(self, n: int, dislikes: list[list[int]]) -> bool:
        graph = [[] for _ in range(n + 1)]
        for u, v in dislikes:
            graph[u].append(v)
            graph[v].append(u)
        group = [-1] * (n + 1)

        def bfs(start):
            queue = deque([start])
            group[start] = 0
            while queue:
                node = queue.popleft()
                for neighbour in graph[node]:
                    if group[neighbour] == -1:
                        group[neighbour] = 1 - group[node]
                        queue.append(neighbour)
                    elif group[neighbour] == group[node]:
                        return False
            return True

        for i in range(1, n + 1):
            if group[i] == -1:
                if not bfs(i):
                    return False
        return True