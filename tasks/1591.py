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

        dependencies = [[] for _ in range(61)]
        for colour in found_colours:
            i = colour * 4
            for row in range(squares[i], squares[i + 1] + 1):
                row_cells = targetGrid[row]
                for col in range(squares[i + 2], squares[i + 3] + 1):
                    inner_colour = row_cells[col]
                    if inner_colour != colour and inner_colour not in dependencies[colour]:
                        dependencies[colour].append(inner_colour)

        cycle_state = [0] * 61

        for colour in found_colours:
            if not cycle_state[colour]:
                stack = [(colour, 0)]
                while stack:
                    inner_colour, i = stack[-1]
                    if i == 0:
                        cycle_state[inner_colour] = 1

                    found_next = False
                    for j in range(i, len(dependencies[inner_colour])):
                        next = dependencies[inner_colour][j]
                        if cycle_state[next] == 1:
                            return False
                        if not cycle_state[next]:
                            stack[-1] = (inner_colour, j + 1)
                            stack.append((next, 0))
                            found_next = True
                            break

                    if not found_next:
                        cycle_state[inner_colour] = 2
                        stack.pop()
        return True
