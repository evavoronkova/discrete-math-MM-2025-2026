import heapq

class Solution:
    def minimumTime(self, n: int, edges: list[list[int]], disappear: list[int]) -> list[int]:
        graph = [[] for _ in range(n)]
        for a, b, w in edges:
            graph[a].append((b, w))
            graph[b].append((a, w))

        # dist[i] = кратчайшее время от 0 до i
        dist = [float('inf')] * n
        dist[0] = 0
        pq = [(0, 0)]  # (время, узел)

        while pq:
            d, node = heapq.heappop(pq)
            if d > dist[node]:
                continue
            for neib, w in graph[node]:
                new_time = d + w
                # добавляем в кучу только если успеваем до исчезновения
                if new_time < disappear[neib] and new_time < dist[neib]:
                    dist[neib] = new_time
                    heapq.heappush(pq, (new_time, neib))

        # заменяем inf на -1
        answer = []
        for i in range(n):
            if dist[i] == float('inf'):
                answer.append(-1)
            else:
                answer.append(dist[i])

        return answer
    