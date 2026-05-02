from typing import List

class Solution:
    def equationsPossible(self, equations: List[str]) -> bool:
        parent = list(range(26))

        def find(x: int) -> int:
            if parent[x] != x:
                parent[x] = find(parent[x])
            return parent[x]

        def union(x: int, y: int) -> None:
            root_x = find(x)
            root_y = find(y)

            if root_x != root_y:
                parent[root_y] = root_x

        for equation in equations:
            if equation[1:3] == "==":
                first = ord(equation[0]) - ord("a")
                second = ord(equation[3]) - ord("a")
                union(first, second)

        for equation in equations:
            if equation[1:3] == "!=":
                first = ord(equation[0]) - ord("a")
                second = ord(equation[3]) - ord("a")

                if find(first) == find(second):
                    return False

        return True