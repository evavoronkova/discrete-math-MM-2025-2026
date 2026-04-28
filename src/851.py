class Solution:
    def loudAndRich(self, richer: list[list[int]], quiet: list[int]) -> list[int]:
        n = len(quiet)

        graph = [[] for _ in range(n)]
        for a, b in richer:
            graph[b].append(a)  # b -> a: a богаче b

        memo = [-1] * n

        def dfs(person):
            if memo[person] != -1:
                return memo[person]

            # начинаем с самого себя
            quietest_person = person

            # смотрим на всех, кто богаче
            for richer_person in graph[person]:
                candidate = dfs(richer_person)  # самый тихий среди тех, кто богаче richer_person
                if quiet[candidate] < quiet[quietest_person]:
                    quietest_person = candidate

            memo[person] = quietest_person
            return quietest_person

        # запускаем dfs для каждого человека
        answer = []
        for i in range(n):
            answer.append(dfs(i))

        return answer
    