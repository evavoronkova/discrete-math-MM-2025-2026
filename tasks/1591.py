class Solution:
    def isPrintable(self, targetGrid: list[list[int]]) -> bool:
        rows, cols = len(targetGrid), len(targetGrid[0])

        squares = [-1] * 244  # Оптимизация памяти: вместо списков в squares подряд хранятся цвета углов прямоугольников
        found_colours = []
        for row in range(rows):
            row_cells = targetGrid[row]
            for col in range(cols):
                colour = row_cells[col]
                i = colour * 4
                if squares[i] == -1:
                    squares[i:i + 4] = [row, row, col, col]
                    found_colours.append(colour)
                else:
                    if row < squares[i]:
                        squares[i] = row
                    if row > squares[i + 1]:
                        squares[i + 1] = row
                    if col < squares[i + 2]:
                        squares[i + 2] = col
                    if col > squares[i + 3]:
                        squares[i + 3] = col

        dependencies = [set() for _ in range(61)]
        for colour in found_colours:
            i = colour * 4
            upper_row, lower_row, left_col, right_col = squares[i], squares[i + 1], squares[i + 2], squares[i + 3]
            for row in range(upper_row, lower_row + 1):
                row_cells = targetGrid[row]
                for col in range(left_col, right_col + 1):
                    inner_colour = row_cells[col]
                    if inner_colour != colour:
                        dependencies[colour].add(inner_colour)

        cycle_state = [0] * 61
        for color in found_colours:
            if cycle_state[color] == 0:
                stack = [(color, iter(dependencies[color]))]
                cycle_state[color] = 1
                while stack:
                    current, neighbors = stack[-1]
                    try:
                        nxt = next(neighbors)
                        if cycle_state[nxt] == 1:
                            return False
                        if cycle_state[nxt] == 0:
                            cycle_state[nxt] = 1
                            stack.append((nxt, iter(dependencies[nxt])))
                    except StopIteration:
                        cycle_state[current] = 2
                        stack.pop()
        return True
