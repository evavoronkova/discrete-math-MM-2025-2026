from collections import deque

class Solution:
    def shortestAlternatingPaths(self, n: int, redEdges: list[list[int]], blueEdges: list[list[int]]) -> list[int]:
        # строим графы по цветам
        red_graph = [[] for _ in range(n)]
        blue_graph = [[] for _ in range(n)]

        for a, b in redEdges:
            red_graph[a].append(b)

        for u, v in blueEdges:
            blue_graph[u].append(v)

        answer = [-1] * n
        answer[0] = 0

        # visited[узел][цвет] — были/нет в этом состоянии
        # 0 = красный последний, 1 = синий последний
        visited = [[False, False] for _ in range(n)]
        visited[0][0] = visited[0][1] = True

        # (узел, цвет последнего, расстояние)
        queue = deque()
        queue.append((0, 0, 0))
        queue.append((0, 1, 0))

        while queue:
            node, last_color, dist = queue.popleft()

            if last_color == 0:  # пришли по красному -> ищем синие
                for neib in blue_graph[node]:
                    if not visited[neib][1]:
                        visited[neib][1] = True
                        if answer[neib] == -1:
                            answer[neib] = dist + 1
                        queue.append((neib, 1, dist + 1))
            else:  # пришли по синему -> ищем красные
                for neib in red_graph[node]:
                    if not visited[neib][0]:
                        visited[neib][0] = True
                        if answer[neib] == -1:
                            answer[neib] = dist + 1
                        queue.append((neib, 0, dist + 1))

        return answer
    