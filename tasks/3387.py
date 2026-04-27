from collections import defaultdict, deque

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

        def bfs(start_curr: str, start_amount: float, graph: defaultdict[str, dict]):
            max_amounts = defaultdict(float)
            max_amounts[start_curr] = start_amount

            queue = deque()
            queue.append((start_curr, start_amount))

            while queue:
                curr, amount = queue.popleft()
                for neighbour, rate in graph[curr].items():
                    new_amount = amount * rate
                    if new_amount > max_amounts[neighbour]:
                        max_amounts[neighbour] = new_amount
                        queue.append((neighbour, new_amount))
            return max_amounts

        amounts1 = bfs(initialCurrency, 1.0, graph1)
        res = 1.0

        for currency, amount in amounts1.items():
            if currency in graph2:
                amounts2 = bfs(currency, amount, graph2)
                res = max(res, amounts2[initialCurrency])

        return res
