from collections import defaultdict


class Solution:
    def isPrintable(self, targetGrid: list[list[int]]) -> bool:
        rows, cols = len(targetGrid), len(targetGrid[0])

        squares = {}
        for row in range(rows):
            for col in range(cols):
                colour = targetGrid[row][col]
                if colour not in squares:
                    squares[colour] = [row, row, col, col]
                else:
                    square = squares[colour]
                    square[0] = row if row < square[0] else square[0]
                    square[1] = row if row > square[1] else square[1]
                    square[2] = col if col < square[2] else square[2]
                    square[3] = col if col > square[3] else square[3]

        dependencies = defaultdict(set)
        for colour, (upper_row, lower_row, left_col, right_col) in squares.items():
            for row in range(upper_row, lower_row + 1):
                for col in range(left_col, right_col + 1):
                    inner_colour = targetGrid[row][col]
                    if inner_colour != colour:
                        dependencies[colour].add(inner_colour)

        cycle_state = defaultdict(int)
        def has_cycle(colour: int) -> bool:
            cycle_state[colour] = 1
            for inner_colour in dependencies[colour]:
                if cycle_state[inner_colour] == 1:
                    return True
                if not cycle_state[inner_colour] and has_cycle(inner_colour):
                    return True
            cycle_state[colour] = 2
            return False

        for colour in squares:
            if not cycle_state[colour]:
                if has_cycle(colour):
                    return False

        return True
