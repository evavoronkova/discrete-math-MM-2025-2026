from typing import List
from collections import deque

class Solution:
    def sumOfDistancesInTree(self, n: int, edges: List[List[int]]) -> List[int]:
        graph = [[] for _ in range(n)]

        for a, b in edges:
            graph[a].append(b)
            graph[b].append(a)

        answer = []

        for start in range(n):
            distance_sum = self.bfs(start, n, graph)
            answer.append(distance_sum)

        return answer

    def bfs(self, start: int, n: int, graph: List[List[int]]) -> int:
        visited = [False] * n
        distance = [0] * n

        visited[start] = True
        queue = deque([start])

        total = 0

        while queue:
            current = queue.popleft()

            for neighbor in graph[current]:
                if not visited[neighbor]:
                    visited[neighbor] = True
                    distance[neighbor] = distance[current] + 1
                    total += distance[neighbor]
                    queue.append(neighbor)

        return total