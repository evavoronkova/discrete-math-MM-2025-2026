from collections import deque

class Solution:
    def minCost(self, grid: list[list[int]]) -> int:
        m, n = len(grid), len(grid[0])

        # направления: стрелка -> (dx, dy) (дельты)
        # 1-вправо, 2-влево, 3-вниз, 4-вверх
        directions = {
            1: (0, 1),
            2: (0, -1),
            3: (1, 0),
            4: (-1, 0)
        }

        # все возможные движения
        moves = [(0, 1), (0, -1), (1, 0), (-1, 0)]

        # cost[i][j] = минимальная стоимость добраться до (i,j)
        cost = [[float('inf')] * n for _ in range(m)]
        cost[0][0] = 0

        dq = deque()
        dq.append((0, 0))

        while dq:
            i, j = dq.popleft()

            # направление стрелки в этой клетке
            arrow = grid[i][j]

            for dx, dy in moves:
                ni = i + dx
                nj = j + dy

                # проверяем границы
                if 0 <= ni < m and 0 <= nj < n:
                    # 0 если идём по стрелке, 1 если нет
                    new_cost = cost[i][j] + (0 if (dx, dy) == directions[arrow] else 1)

                    if new_cost < cost[ni][nj]:
                        cost[ni][nj] = new_cost
                        if (dx, dy) == directions[arrow]:
                            dq.appendleft((ni, nj))  # вес 0 -> доб. в начало (в приоритете)
                        else:
                            dq.append((ni, nj))  # вес 1 -> в конец

        return cost[m - 1][n - 1]
