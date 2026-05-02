from typing import List
import sys

class Solution:
    def sumOfDistancesInTree(self, n: int, edges: List[List[int]]) -> List[int]:
        sys.setrecursionlimit(10 ** 6)

        graph = [[] for _ in range(n)]

        for a, b in edges:
            graph[a].append(b)
            graph[b].append(a)

        count = [1] * n
        answer = [0] * n

        def dfs1(node: int, parent: int) -> None:
            for child in graph[node]:
                if child == parent:
                    continue

                dfs1(child, node)

                count[node] += count[child]
                answer[node] += answer[child] + count[child]

        def dfs2(node: int, parent: int) -> None:
            for child in graph[node]:
                if child == parent:
                    continue

                answer[child] = answer[node] - count[child] + (n - count[child])
                dfs2(child, node)

        dfs1(0, -1)
        dfs2(0, -1)

        return answer