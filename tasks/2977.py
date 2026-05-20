import math

INF = math.inf


class Solution:
    def minimumCost(self, source: str, target: str, original: list[str], changed: list[str], cost: list[int]) -> int:
        substrings = list(set(original) | set(changed))
        substr_id = {substring: i for i, substring in enumerate(substrings)}  # нумеруем все подстроки
        substr_num = len(substrings)

        # строим граф замен подстрок: узлы - подстроки, ребра с минимальной стоимостью - доступные замены
        costs = [[INF] * substr_num for _ in range(substr_num)]
        for i in range(substr_num):
            costs[i][i] = 0

        # сначала просто записываем замены
        for i in range(len(original)):
            old, new, c = original[i], changed[i], cost[i]
            j, k = substr_id[old], substr_id[new]
            if c < costs[j][k]:
                costs[j][k] = c

        # теперь ищем минимальные пути из i в j через все доступные k - алгоритм Флойда-Уоршелла
        for k in range(substr_num):
            cost_k = costs[k]
            for i in range(substr_num):
                cost_i = costs[i]
                if cost_i[k] == INF:
                    continue
                for j in range(substr_num):
                    if cost_i[k] + cost_k[j] < cost_i[j]:
                        cost_i[j] = cost_i[k] + cost_k[j]

        lengths = sorted(list(set(len(s) for s in original)))  # записываем длины подстрок, которые можно заменить

        num = len(source)
        dynamic_costs = [INF] * (num + 1)  # динамический массив для нахождения оптимальных цен замен подстрок
        dynamic_costs[0] = 0

        for i in range(num):
            if dynamic_costs[i] == INF:  # не можем дойти до этого символа
                continue
            if source[i] == target[i]:  # данный символ совпадает с целью, шагаем вперед бесплатно
                if dynamic_costs[i] < dynamic_costs[i + 1]:
                    dynamic_costs[i + 1] = dynamic_costs[i]

            for length in lengths:  # перебираем всевозможные варианты длин подстрок
                if i + length > num:
                    break
                old = source[i: i + length]
                new = target[i: i + length]

                j = substr_id.get(old)
                k = substr_id.get(new)

                if j is not None and k is not None:
                    cost_jk = costs[j][k]
                    if cost_jk != INF:  # находим возможную замену подстроки и сравниваем цену
                        if dynamic_costs[i] + cost_jk < dynamic_costs[i + length]:
                            dynamic_costs[i + length] = dynamic_costs[i] + cost_jk

        return dynamic_costs[num] if dynamic_costs[num] != INF else -1
