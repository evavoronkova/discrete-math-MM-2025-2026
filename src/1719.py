class Solution:
    def checkWays(self, pairs: list[list[int]]) -> int:
        # собираем уникальные вершины и нумеруем их
        nodes_set = set()
        for x, y in pairs:
            nodes_set.add(x)
            nodes_set.add(y)
        nodes_list = sorted(nodes_set)
        # создаём словарь index: index[val] = i
        index = {val: i for i, val in enumerate(nodes_list)}
        n = len(nodes_list)

        # строим матрицу смежности и считаем степени - количество связей с другими узлами
        graph = [[False] * n for _ in range(n)]
        neighbors = [[] for _ in range(n)]
        for x, y in pairs:
            i, j = index[x], index[y]
            graph[i][j] = True
            graph[j][i] = True
            neighbors[i].append(j)
            neighbors[j].append(i)

        degree = [0] * n
        for i in range(n):
            degree[i] = sum(graph[i])

        # количество узлов
        n = len(graph)

        # сортируем узлы по убыванию степени
        nodes = list(range(n))
        nodes.sort(key=lambda node: degree[node], reverse=True)

        # корень должен быть связан со всеми
        if degree[nodes[0]] != n - 1:
            return 0

        ways = 1

        # для каждого узла ищем родителя
        for i in range(n):
            node = nodes[i]

            # ищем среди соседей того, у кого степень больше (родитель)
            parent = None
            parent_degree = float('inf')
            for neigh in neighbors[node]:
                if degree[node] <= degree[neigh] < parent_degree:
                    parent = neigh
                    parent_degree = degree[neigh]

            if parent is None:
                continue  # корень

            # проверяем, что все соседи node являются соседями родителя (т.к он общий предок)
            for neigh in neighbors[node]:
                if neigh != parent and not graph[parent][neigh]:
                    return 0

            # если степени совпадают, не можем сказать кто из двух родитель
            # и может быть больше одного способа
            if degree[parent] == degree[node]:
                ways = 2

        return ways
