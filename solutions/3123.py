import heapq

class Solution:
    def findAnswer(self, n: int, edges: List[List[int]]) -> List[bool]:
        data = [[] for _ in range(n)]
        distance = [float("inf") for _ in range(n)]
        distance[0] = 0
        pq = [(0, 0)]
        
        for i, (u_node, v_node, weight) in enumerate(edges):
            data[u_node].append((v_node, weight, i))
            data[v_node].append((u_node, weight, i))

        while pq:
            d, u = heapq.heappop(pq)

            if d > distance[u]:
                continue

            for v, weight, edge in data[u]:
                new_distance = d + weight

                if new_distance < distance[v]:
                    distance[v] = new_distance
                    heapq.heappush(pq, (new_distance, v))

        ans = [False] * len(edges)
        
        if distance[n - 1] == float("inf"):
            return ans
            
        queue = [n - 1]
        visited = {n - 1} 
        
        while queue:
            curr = queue.pop()
            
            for prev, weight, edge_index in data[curr]:

                if distance[prev] + weight == distance[curr]:
                    ans[edge_index] = True

                    if prev not in visited:
                        visited.add(prev)
                        queue.append(prev)

        return ans
