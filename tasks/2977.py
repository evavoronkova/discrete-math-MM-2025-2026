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

        trie = {}
        for substring in substrings:
            node = trie
            for char in substring:
                node = node.setdefault(char, {})
            node['id'] = substr_id[substring]


        num = len(source)
        dynamic_costs = [INF] * (num + 1)
        dynamic_costs[0] = 0

        for i in range(num):
            if dynamic_costs[i] == INF:
                continue
            if source[i] == target[i]:
                dynamic_costs[i + 1] = min(dynamic_costs[i + 1], dynamic_costs[i])

            old = trie
            new = trie
            for j in range(i, num):
                if source[j] not in old or target[j] not in new:
                    break
                old = old[source[j]]
                new = new[target[j]]

                if 'id' in old and 'id' in new:
                    k1, k2 = old['id'], new['id']
                    if costs[k1][k2] != INF:
                        dynamic_costs[j + 1] = min(dynamic_costs[j + 1], dynamic_costs[i] + costs[k1][k2])

        return dynamic_costs[num] if dynamic_costs[num] != INF else -1
