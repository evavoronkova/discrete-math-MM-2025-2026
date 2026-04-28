class Solution:
    def minScore(self, n: int, roads: list[list[int]]) -> int:
        # строим из графа 2 списка смежности,
        # так как кортежи хранить накладно по памяти
        graph_neighbors = [[] for _ in range(n + 1)]
        graph_weights = [[] for _ in range(n + 1)]
        for a, b, dist in roads:
            graph_neighbors[a].append(b)
            graph_weights[a].append(dist)
            graph_neighbors[b].append(a)
            graph_weights[b].append(dist)

        visited = [False] * (n + 1)
        min_edge = 10**4
        stack = [1]
        visited[1] = True

        while stack:
            city = stack.pop()
            for i in range(len(graph_neighbors[city])):
                dist = graph_weights[city][i]
                if dist < min_edge:
                    min_edge = dist
                neib = graph_neighbors[city][i]
                if not visited[neib]:
                    visited[neib] = True
                    stack.append(neib)

        return min_edge
