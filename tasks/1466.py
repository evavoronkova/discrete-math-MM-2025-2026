class Solution:
    def minReorder(self, n: int, connections: list[list[int]]) -> int:
        roads = connections.copy()
        to_cities = [0]
        min_changed = 0

        while roads:
            for pair in roads:
                a, b = pair
                if a in to_cities:
                    min_changed += 1
                    to_cities.append(b)
                    roads.remove(pair)
                elif b in to_cities:
                    to_cities.append(a)
                    roads.remove(pair)

        return min_changed