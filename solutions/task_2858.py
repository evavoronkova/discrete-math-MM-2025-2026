class Solution:
    def minEdgeReversals(self, n: int, edges: list[list[int]]) -> list[int]:
        graph = [[] for _ in range(n)]
        for u, v in edges:
            graph[u].append((v, 1))
            graph[v].append((u, -1))
        
        def dfs(node, parent):
            cost = 0
            for nei, w in graph[node]:
                if nei != parent:
                    if w == -1:
                        cost += 1
                    cost += dfs(nei, node)
            return cost
        
        base = dfs(0, -1)
        ans = [0] * n
        ans[0] = base
        
        def reroot(node, parent, prev_cost):
            for nei, w in graph[node]:
                if nei != parent:
                    if w == 1:
                        ans[nei] = prev_cost + 1
                    else:
                        ans[nei] = prev_cost - 1
                    reroot(nei, node, ans[nei])
        
        reroot(0, -1, base)
        return ans