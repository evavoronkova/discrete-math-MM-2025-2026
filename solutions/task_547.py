class Solution:
    def findCircleNum(self, isConnected: list[list[int]]) -> int:
        n = len(isConnected)
        visited = [False] * n
        
        def dfs(start):
            visited[start] = True
            stack = [start]
            while stack:
                node = stack.pop()
                for neighbour in range(n):
                    if isConnected[node][neighbour] == 1 and not visited[node][neighbour]:
                        visited[neighbour] = True
                        stack.append(neighbour)
        
        provinces = 0
        for i in range(n):
            if not visited[i]:
                dfs(i)
                provinces += 1
        return provinces