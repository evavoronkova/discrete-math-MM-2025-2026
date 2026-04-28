import math

INF = math.inf


class Solution:
    def minimumCost(self, source: str, target: str, original: list[str], changed: list[str], cost: list[int]) -> int:
        substrings = list(set(original) | set(changed))
        substr_id = {substring: i for i, substring in enumerate(substrings)}
        substr_num = len(substrings)

        costs = [[INF] * substr_num for _ in range(substr_num)]
        for i in range(substr_num):
            costs[i][i] = 0

        for i in range(len(original)):
            old, new, c = original[i], changed[i], cost[i]
            j, k = substr_id[old], substr_id[new]
            costs[j][k] = min(costs[j][k], c)

        for k in range(substr_num):
            for i in range(substr_num):
                for j in range(substr_num):
                    costs[i][j] = min(costs[i][j], costs[i][k] + costs[k][j])

        num = len(source)
        dynamic_costs = [INF] * (num + 1)
        dynamic_costs[0] = 0

        lengths = sorted(list(set(len(s) for s in original)))

        for i in range(num):
            if dynamic_costs[i] == INF:
                continue
            if source[i] == target[i]:
                dynamic_costs[i + 1] = min(dynamic_costs[i + 1], dynamic_costs[i])

            for length in lengths:
                if i + length <= num:
                    old = source[i:i + length]
                    new = target[i:i + length]
                    if old in substr_id and new in substr_id:
                        j, k = substr_id[old], substr_id[new]
                        if costs[j][k] != INF:
                            dynamic_costs[i + length] = min(dynamic_costs[i + length], dynamic_costs[i] + costs[j][k])
        return dynamic_costs[num] if dynamic_costs[num] != INF else -1
