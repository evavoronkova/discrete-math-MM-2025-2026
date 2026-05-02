from typing import List
from collections import deque

class Solution:
    def largestPathValue(self, colors: str, edges: List[List[int]]) -> int:
        n = len(colors)

        graph = [[] for _ in range(n)]
        indegree = [0] * n

        for a, b in edges:
            graph[a].append(b)
            indegree[b] += 1

        dp = [[0] * 26 for _ in range(n)]

        queue = deque()

        for i in range(n):
            if indegree[i] == 0:
                queue.append(i)

        visited_count = 0
        answer = 0

        while queue:
            current = queue.popleft()
            visited_count += 1

            color_index = ord(colors[current]) - ord("a")
            dp[current][color_index] += 1

            answer = max(answer, max(dp[current]))

            for neighbor in graph[current]:
                for c in range(26):
                    dp[neighbor][c] = max(dp[neighbor][c], dp[current][c])

                indegree[neighbor] -= 1

                if indegree[neighbor] == 0:
                    queue.append(neighbor)

        if visited_count != n:
            return -1

        return answer