from collections import defaultdict

class Solution:
    def maxAmount(self, initialCurrency: str, pairs1: list[list[str]], rates1: list[float], pairs2: list[list[str]],
                  rates2: list[float]) -> float:
        graph1 = defaultdict(dict)
        graph2 = defaultdict(dict)

        for i in range(len(pairs1)):
            start, target = pairs1[i]
            rate = rates1[i]
            graph1[start][target] = rate
            graph1[target][start] = 1.0 / rate

        for i in range(len(pairs2)):
            start, target = pairs2[i]
            rate = rates2[i]
            graph2[start][target] = rate
            graph2[target][start] = 1.0 / rate

        amounts2 = [1.0]
        amounts1 = defaultdict(float)

        def dfs1(curr: str, amount: float, visited: list[str]):
            visited.append(curr)
            amounts1[curr] = amount

            for neighbour, rate in graph1[curr].items():
                if neighbour not in visited:
                    dfs1(neighbour, amount * rate, visited)
                    visited.pop()

        def dfs2(curr: str, amount: float, visited: list[str]):
            visited.append(curr)

            if curr == initialCurrency:
                amounts2.append(amount)
                return

            for neighbour, rate in graph2[curr].items():
                if neighbour not in visited:
                    dfs2(neighbour, amount * rate, visited)
                    visited.pop()

        dfs1(initialCurrency, 1.0, [])

        for currency, amount in amounts1.items():
            if currency in graph2:
                dfs2(currency, amount, [])

        return max(amounts2)
