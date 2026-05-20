from collections import defaultdict
import heapq


class Solution:
    def maxProbability(self, n: int, edges: list[list[int]], succProb: list[float], start_node: int, end_node: int) -> float:
        # строим граф с n вершинами и с ребрами, имеющими вес - вероятность успешного прохождения
        graph = defaultdict(dict)

        for i in range(len(edges)):
            first, second = edges[i]
            prob = succProb[i]
            graph[first][second] = prob
            graph[second][first] = prob

        probs = [0.0] * n
        probs[start_node] = 1.0

        queue = [(-1.0, start_node)]

        # ищем наиболее вероятный успешный путь, используя алгоритм Дейкстры с очередью с приоритетом
        while queue:
            prob, node = heapq.heappop(queue)
            prob = -prob  # heapq реализует приоритет минимума, поэтому кладем отрицательные вероятности

            if node == end_node:
                return prob

            if prob < probs[node]:
                continue

            for neighbour, new_prob in graph[node].items():
                if prob * new_prob > probs[neighbour]:
                    probs[neighbour] = prob * new_prob  # находим новую большую вероятность и записываем
                    heapq.heappush(queue, (-probs[neighbour], neighbour))
        return 0.0
