class Solution:
    def remainingMethods(self, n: int, k: int, invocations: list[list[int]]) -> list[int]:
        graph = [[] for _ in range(n)]  # исходящие вызовы
        reverse_graph = [[] for _ in range(n)]  # входящие вызовы

        for a, b in invocations:
            graph[a].append(b)  # a вызывает b
            reverse_graph[b].append(a)  # b вызывается из a

        # собираем все подозрительные методы
        suspicious = [False] * n
        stack = [k]
        suspicious[k] = True

        while stack:
            node = stack.pop()
            for neib in graph[node]:
                if not suspicious[neib]:
                    suspicious[neib] = True
                    stack.append(neib)

        # проверяем, есть ли вызов подозрительных методов извне
        for i in range(n):
            if suspicious[i]:  # подозрительный метод
                for neib in reverse_graph[i]:
                    if not suspicious[neib]:
                        return list(range(n))  # нельзя удалить -> все методы остаются

        return [i for i in range(n) if not suspicious[i]]
