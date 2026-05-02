from typing import List
from collections import deque

class Solution:
    def networkBecomesIdle(self, edges: List[List[int]], patience: List[int]) -> int:
        n = len(patience)

        graph = [[] for _ in range(n)]

        for a, b in edges:
            graph[a].append(b)
            graph[b].append(a)

        distance = [-1] * n
        distance[0] = 0

        queue = deque([0])

        while queue:
            current = queue.popleft()

            for neighbor in graph[current]:
                if distance[neighbor] == -1:
                    distance[neighbor] = distance[current] + 1
                    queue.append(neighbor)

        answer = 0

        for server in range(1, n):
            round_trip_time = distance[server] * 2

            if patience[server] >= round_trip_time:
                last_send_time = 0
            else:
                last_send_time = ((round_trip_time - 1) // patience[server]) * patience[server]

            last_reply_time = last_send_time + round_trip_time
            answer = max(answer, last_reply_time)

        return answer + 1