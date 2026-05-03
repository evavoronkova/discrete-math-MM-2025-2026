class Solution:
    def makeConnected(self, n: int, connections: list[list[int]]) -> int:
        if len(connections) < n - 1:
            return -1
        graph = [[] for _ in range(n)]
        for u, v in connections:
            graph[u].append(v)
            graph[v].append(u)
        visited = [False] * n
        
        def dfs(node):
            stack = [node]
            while stack:
                u = stack.pop()
                for v in graph[u]:
                    if not visited[v]:
                        visited[v] = True
                        stack.append(v)
        
        components = 0
        for i in range(n):
            if not visited[i]:
                components += 1
                visited[i] = True
                dfs(i)
        return components - 1