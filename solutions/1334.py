import heapq

def findTheCity(n: int, edges: list[list[int]], distanceThreshold: int) -> int:
    graph = [[] for _ in range(n)]
    for u, v, w in edges:
        graph[u].append((v, w))
        graph[v].append((u, w))

    def get_reachable_count(start: int) -> int:
        dist = [float('inf')] * n
        dist[start] = 0
        pq = [(0, start)]
        
        while pq:
            d, u = heapq.heappop(pq)
            if d > dist[u]:
                continue
                
            for v, weight in graph[u]:
                new_d = d + weight
                if new_d <= distanceThreshold and new_d < dist[v]:
                    dist[v] = new_d
                    heapq.heappush(pq, (new_d, v))
        
        return sum(1 for d in dist if d <= distanceThreshold) - 1

    min_reachable = float('inf')
    best_city = -1
    
    for i in range(n):
        reachable = get_reachable_count(i)
        if reachable <= min_reachable:
            min_reachable = reachable
            best_city = i
            
    return best_city


