from collections import deque


class Solution:
    def isBipartite(self, graph: list[list[int]]) -> bool:
        colours = [0] * len(graph)
        queue = deque()

        for i in range(len(graph)):
            if not colours[i]:
                colours[i] = 1
                queue.append(i)

                while queue:
                    curr = queue.popleft()
                    for neighbour in graph[curr]:
                        if not colours[neighbour]:
                            colours[neighbour] = 3 - colours[curr]
                            queue.append(neighbour)
                        elif colours[neighbour] == colours[curr]:
                            return False
        return True
