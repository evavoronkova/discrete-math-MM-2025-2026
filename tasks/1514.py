from collections import defaultdict

class Solution:
    def maxProbability(self, n: int, edges: list[list[int]], succProb: list[float], start_node: int, end_node: int) -> float:
        graph = defaultdict(dict)

        for i in range(len(edges)):
            first, second = edges[i]
            prob = succProb[i]
            graph[first][second] = prob

        probs = [0.0]

        def dfs(current: int, prob: float):
            if current == end_node:
                probs.append(prob)
                return

            for neighbour, next_prob in graph[current].items():
                dfs(neighbour, prob * next_prob)

        dfs(start_node, 1.0)
        return max(probs)
