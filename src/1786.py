import heapq

class Solution:
    def countRestrictedPaths(self, n: int, edges: list[list[int]]) -> int:
        MOD = 10 ** 9 + 7

        # строим список смежности
        graph = [[] for _ in range(n + 1)]
        for a, b, w in edges:
            graph[a].append((b, w))
            graph[b].append((a, w))

        # dist[i] = кратчайшее расстояние от i до n
        dist = [float('inf')] * (n + 1)
        dist[n] = 0
        pq = [(0, n)]  # (расстояние, узел)

        while pq:
            d, node = heapq.heappop(pq)  # достаём ближайший к n узел
            if d > dist[node]:
                continue
            for neib, w in graph[node]:
                new_dist = d + w
                if new_dist < dist[neib]:  # нашли путь короче
                    dist[neib] = new_dist
                    heapq.heappush(pq, (new_dist, neib))  # добавляем в кучу

        memo = [-1] * (n + 1)  # memo[i] = сколько restricted paths из i в n

        def dfs(node):
            if node == n:
                return 1  # из n в n 1 путь
            if memo[node] != -1:
                return memo[node]  # уже считали

            total = 0
            for neib, _ in graph[node]:
                if dist[node] > dist[neib]:
                    total = (total + dfs(neib)) % MOD

            memo[node] = total
            return total

        return dfs(1)
