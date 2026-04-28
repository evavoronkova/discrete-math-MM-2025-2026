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
            if c < costs[j][k]:
                costs[j][k] = c

        for k in range(substr_num):
            cost_k = costs[k]
            for i in range(substr_num):
                cost_i = costs[i]
                if cost_i[k] == INF:
                    continue
                for j in range(substr_num):
                    if cost_i[k] + cost_k[j] < cost_i[j]:
                        cost_i[j] = cost_i[k] + cost_k[j]

        lengths = sorted(list(set(len(s) for s in original)))

        num = len(source)
        dynamic_costs = [INF] * (num + 1)
        dynamic_costs[0] = 0

        for i in range(num):
            if dynamic_costs[i] == INF:
                continue
            if source[i] == target[i]:
                if dynamic_costs[i] < dynamic_costs[i + 1]:
                    dynamic_costs[i + 1] = dynamic_costs[i]

            for length in lengths:
                if i + length > num:
                    break
                old = source[i: i + length]
                new = target[i: i + length]

                j = substr_id.get(old)
                k = substr_id.get(new)

                if j is not None and k is not None:
                    cost_jk = costs[j][k]
                    if cost_jk != INF:
                        if dynamic_costs[i] + cost_jk < dynamic_costs[i + length]:
                            dynamic_costs[i + length] = dynamic_costs[i] + cost_jk

        return dynamic_costs[num] if dynamic_costs[num] != INF else -1
