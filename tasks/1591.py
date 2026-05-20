class Solution:
    def isPrintable(self, targetGrid: list[list[int]]) -> bool:
        rows, cols = len(targetGrid), len(targetGrid[0])

        # записываем в squares границы прямоугольников всех цветов (1 нулевой индекс + 60 цветов) * 4
        squares = [-1] * 244  # Оптимизация памяти: вместо списков в squares подряд хранятся цвета углов прямоугольников
        found_colours = []
        for row in range(rows):
            row_cells = targetGrid[row]
            for col in range(cols):
                colour = row_cells[col]
                i = colour * 4
                if squares[i] == -1:  # нашли новый цвет и записываем клетку с ним
                    squares[i:i + 4] = [row, row, col, col]
                    found_colours.append(colour)
                else:
                    if row < squares[i]:
                        squares[i] = row  # нашли новую верхнюю границу
                    if row > squares[i + 1]:
                        squares[i + 1] = row  # нашли новую нижнюю границу
                    if col < squares[i + 2]:
                        squares[i + 2] = col  # нашли новую левую границу
                    if col > squares[i + 3]:
                        squares[i + 3] = col  # нашли новую правую границу

        dependencies = [set() for _ in range(61)]  # составляем ориентированный граф зависимостей цветов
        for colour in found_colours:
            i = colour * 4
            upper_row, lower_row, left_col, right_col = squares[i], squares[i + 1], squares[i + 2], squares[i + 3]
            for row in range(upper_row, lower_row + 1):
                row_cells = targetGrid[row]
                for col in range(left_col, right_col + 1):
                    inner_colour = row_cells[col]
                    if inner_colour != colour:  # нашли вложенный цвет, добавляем его в зависимость от текущего
                        dependencies[colour].add(inner_colour)

        # ищем циклы в графе с итеративным dfs
        cycle_state = [0] * 61  # массив с состояниями проверки цветов: 0 - не смотрели, 1 - проверяем, 2 - прошли
        for color in found_colours:
            if cycle_state[color] == 0:
                stack = [(color, iter(dependencies[color]))]
                cycle_state[color] = 1
                while stack:
                    current, neighbors = stack[-1]
                    try:
                        nxt = next(neighbors)  # смотрим соседей
                        if cycle_state[nxt] == 1:  # обнаружили цикл, то есть цикличную зависимость цветов
                            return False
                        if cycle_state[nxt] == 0:  # новый цвет
                            cycle_state[nxt] = 1
                            stack.append((nxt, iter(dependencies[nxt])))
                    except StopIteration:
                        cycle_state[current] = 2  # прошли всех соседей, помечаем цвет как проверенный
                        stack.pop()
        return True
