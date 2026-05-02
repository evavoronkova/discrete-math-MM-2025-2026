from typing import List
import heapq

class Solution:
    def maxProbability(self, n: int, edges: List[List[int]], succProb: List[float], start_node: int, end_node: int) -> float:
        graph = [[] for _ in range(n)]

        for i in range(len(edges)):
            a, b = edges[i]
            probability = succProb[i]

            graph[a].append((b, probability))
            graph[b].append((a, probability))

        best = [0.0] * n
        best[start_node] = 1.0

        heap = [(-1.0, start_node)]

        while heap:
            current_prob, current_node = heapq.heappop(heap)
            current_prob = -current_prob

            if current_node == end_node:
                return current_prob

            if current_prob < best[current_node]:
                continue

            for next_node, edge_prob in graph[current_node]:
                new_prob = current_prob * edge_prob

                if new_prob > best[next_node]:
                    best[next_node] = new_prob
                    heapq.heappush(heap, (-new_prob, next_node))

        return 0.0