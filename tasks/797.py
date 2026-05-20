class Solution:
    def allPathsSourceTarget(self, graph: list[list[int]]) -> list[list[int]]:
        result = []
        target = len(graph) - 1

        # dfs строит всевозможные пути до цели
        def dfs(current: int, path: list[int]):
            if current == target:
                result.append(path.copy())
                return

            for neighbour in graph[current]:
                path.append(neighbour)
                dfs(neighbour, path)
                path.pop()  # если поиск не дойдет до цели, возвращаемся назад

        dfs(0, [0])
        return result
