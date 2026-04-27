from collections import defaultdict

class Solution:
    def maxProbability(self, n: int, edges: list[list[int]], succProb: list[float], start_node: int, end_node: int) -> float:
        graph = defaultdict(dict)

        for i in range(len(edges)):
            first, second = edges[i]
            prob = succProb[i]
            graph[first][second] = prob
            graph[second][first] = prob

        probs = [0.0]

        def dfs(current: int, prob: float, visited: list[int]):
            if current == end_node:
                probs.append(prob)
                return

            visited.append(current)
            for neighbour, next_prob in graph[current].items():
                if neighbour not in visited:
                    dfs(neighbour, prob * next_prob, visited)

        dfs(start_node, 1.0, [])
        return max(probs)
