from collections import deque


class Solution:
    def possibleBipartition(self, n: int, dislikes: list[list[int]]) -> bool:
        graph: list[list[int]] = [[] for _ in range(n + 1)]
        for a, b in dislikes:
            graph[a].append(b)
            graph[b].append(a)

        groups = [0] * (n + 1)
        queue = deque()

        for i in range(1, n + 1):
            if not groups[i]:
                groups[i] = 1
                queue.append(i)

                while queue:
                    current = queue.popleft()
                    for neighbour in graph[current]:
                        if not groups[neighbour]:
                            groups[neighbour] = 3 - groups[current]
                            queue.append(neighbour)
                        elif groups[neighbour] == groups[current]:
                            return False

        return True
