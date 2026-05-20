from collections import deque


class Solution:
    def possibleBipartition(self, n: int, dislikes: list[list[int]]) -> bool:
        # создаем граф, где вершины - люди, а ребра - антипатия
        graph: list[list[int]] = [[] for _ in range(n + 1)]
        for a, b in dislikes:
            graph[a].append(b)
            graph[b].append(a)

        groups = [0] * (n + 1)  # будем разбивать вершины на две группы: 1 и 2
        queue = deque()

        # bfs проходит по вершинам, разделяя их на 2 группы (как цвета)
        for i in range(1, n + 1):
            if not groups[i]:
                groups[i] = 1
                queue.append(i)

                while queue:
                    current = queue.popleft()
                    for neighbour in graph[current]:
                        if not groups[neighbour]:
                            groups[neighbour] = 3 - groups[current]  # добавляем соседа в противоположную группу
                            queue.append(neighbour)
                        # если в одной группе два человека не ладят - нет двудольности
                        elif groups[neighbour] == groups[current]:
                            return False
        return True
