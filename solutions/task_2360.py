class Solution:
    def longestCycle(self, edges: list[int]) -> int:
        n = len(edges)
        visited = [False] * n
        in_stack = [-1] * n
        ans = -1
        
        for i in range(n):
            if not visited[i]:
                node = i
                time = 0
                stack = []
                
                while node != -1 and not visited[node]:
                    visited[node] = True
                    in_stack[node] = time
                    stack.append(node)
                    node = edges[node]
                    time += 1
                
                if node != -1 and in_stack[node] != -1:
                    cycle_len = time - in_stack[node]
                    ans = max(ans, cycle_len)
                
                for v in stack:
                    in_stack[v] = -1
        
        return ans