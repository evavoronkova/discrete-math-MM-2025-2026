from typing import List
from collections import deque

class Solution:
    def shortestDistanceAfterQueries(self, n: int, queries: List[List[int]]) -> List[int]:
        graph = [[] for _ in range(n)]

        for i in range(n - 1):
            graph[i].append(i + 1)

        answer = []

        for u, v in queries:
            graph[u].append(v)

            shortest = self.bfs(n, graph)
            answer.append(shortest)

        return answer

    def bfs(self, n: int, graph: List[List[int]]) -> int:
        distance = [-1] * n
        distance[0] = 0

        queue = deque([0])

        while queue:
            current = queue.popleft()

            if current == n - 1:
                return distance[current]

            for neighbor in graph[current]:
                if distance[neighbor] == -1:
                    distance[neighbor] = distance[current] + 1
                    queue.append(neighbor)

        return -1