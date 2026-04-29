import sys

class Solution:
    def isPrintable(self, targetGrid: list[list[int]]) -> bool:
        sys.setrecursionlimit(2000)
        rows, cols = len(targetGrid), len(targetGrid[0])

        squares = [None] * 61
        found_colours = []
        for row in range(rows):
            row_cells = targetGrid[row]
            for col in range(cols):
                colour = row_cells[col]
                if squares[colour] is None:
                    squares[colour] = [row, row, col, col]
                    found_colours.append(colour)
                else:
                    square = squares[colour]
                    if row < square[0]:
                        square[0] = row
                    if row > square[1]:
                        square[1] = row
                    if col < square[2]:
                        square[2] = col
                    if col > square[3]:
                        square[3] = col

        dependencies = [set() for _ in range(61)]
        for colour in found_colours:
            upper_row, lower_row, left_col, right_col = squares[colour]
            for row in range(upper_row, lower_row + 1):
                row_cells = targetGrid[row]
                for col in range(left_col, right_col + 1):
                    inner_colour = row_cells[col]
                    if inner_colour != colour:
                        dependencies[colour].add(inner_colour)

        cycle_state = [0] * 61
        def has_cycle(colour: int) -> bool:
            cycle_state[colour] = 1
            for inner_colour in dependencies[colour]:
                if cycle_state[inner_colour] == 1:
                    return True
                if not cycle_state[inner_colour] and has_cycle(inner_colour):
                    return True
            cycle_state[colour] = 2
            return False

        for colour in found_colours:
            if not cycle_state[colour]:
                if has_cycle(colour):
                    return False

        return True
