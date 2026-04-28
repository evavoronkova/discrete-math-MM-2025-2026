class Solution:
    def allPathsSourceTarget(self, graph: list[list[int]]) -> list[list[int]]:
        result = []
        path = [0]
        target = len(graph)-1

        def dfs(node):
            if node == target:
                result.append(path[:])
                return

            for neib in graph[node]:
                path.append(neib)
                dfs(neib)
                path.pop()

        dfs(0)
        return result
