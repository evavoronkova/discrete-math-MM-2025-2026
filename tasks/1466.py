from collections import defaultdict


class Solution:
    def minReorder(self, n: int, connections: list[list[int]]) -> int:
        graph = defaultdict(dict)
        to_cities = [0]
        min_changed = 0


        for a, b in connections:
            if a in to_cities:
                min_changed += 1
                to_cities.append(b)
            elif b in to_cities:
                to_cities.append(a)

        return min_changed
