from collections import deque


class Solution:
    def isBipartite(self, graph: list[list[int]]) -> bool:
        colours = [0] * len(graph)  # будем раскрашивать вершины в два "цвета": 1 и 2, по умолчанию 0 - пока нет цвета
        queue = deque()

        # bfs проходит по вершинам, раскрашивая их по двум цветам
        for i in range(len(graph)):
            if not colours[i]:
                colours[i] = 1
                queue.append(i)

                while queue:
                    curr = queue.popleft()
                    for neighbour in graph[curr]:
                        if not colours[neighbour]:
                            colours[neighbour] = 3 - colours[curr]  # красим соседа в противоположный цвет текущей вершины
                            queue.append(neighbour)
                        # если цвет соседа совпадает с текущим цветом - нет двудольности
                        elif colours[neighbour] == colours[curr]:
                            return False
        return True
