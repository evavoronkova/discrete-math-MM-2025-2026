from collections import defaultdict
from typing import List


class Solution:
    def calcEquation(self, equations: List[List[str]], values: List[float], queries: List[List[str]]) -> List[float]:
        graph = defaultdict(dict)

        for i in range(len(equations)):
            a, b = equations[i]
            val = values[i]
            graph[a][b] = val
            graph[b][a] = 1.0 / val

        def dfs(current: str, target: str, visited: set[str]) -> float:
            if current not in graph or target not in graph:
                return -1.0
            if current == target:
                return 1.0

            visited.add(current)
            for neighbour, weight in graph[current].items():
                if neighbour not in visited:
                    res = dfs(neighbour, target, visited)
                    if res != -1.0:
                        return weight * res
            return -1.0

        result: List[float] = []
        for start, end in queries:
            if start not in graph or end not in graph:
                result.append(-1.0)
            else:
                result.append(dfs(start, end, set()))
        return result

