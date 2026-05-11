class Solution:
    def criticalConnections(self, n: int, connections: list[list[int]]) -> list[list[int]]:
        graph = [[] for _ in range(n)]
        for u, v in connections:
            graph[u].append(v)
            graph[v].append(u)
            
        tin = [-1] * n
        low = [-1] * n
        timer = 0
        ans = []
        
        def dfs(node, parent):
            nonlocal timer
            tin[node] = low[node] = timer
            timer += 1
            
            for neighbor in graph[node]:
                if neighbor == parent:
                    continue
                    
                if tin[neighbor] != -1:
                    low[node] = min(low[node], tin[neighbor])
                else:
                    dfs(neighbor, node)
                    
                    
                    if low[neighbor] > tin[node]:
                        ans.append([node, neighbor])
                        
        dfs(0, -1)
        
        return ans