from enum import IntEnum
from functools import partial


class Axis(IntEnum):
    X = 0
    Y = 1


noline_print = partial(print, end="")

with open("input.txt") as f:
    dots = []
    instructions = []

    coord_sep = ","
    instruction_sep = "="

    for line in f:
        if coord_sep in line:
            coords = line.split(coord_sep)
            # Use a list for the 'dots' as well as each 'dot' since we modify each
            # dot constantly through folding. A tuple would require constant conversions.
            # Dots is a list and not a set for now since we cannot hold lists (each dot)
            # inside sets (lists are not hashable). Ignore duplicate dots until the end.
            dots.append([int(coords[Axis.X]), int(coords[Axis.Y])])
        elif instruction_sep in line:
            component, at = line.split()[-1].split(instruction_sep)
            instructions.append((Axis.X if component == "x" else Axis.Y, int(at)))


def show(grid):
    max_x = max(x for x, _ in grid)
    max_y = max(y for _, y in grid)
    for y in range(max_y + 1):
        for x in range(max_x + 1):
            if [x, y] in grid:
                noline_print("#")
            else:
                noline_print(" ")  # Space is much more readable than period
        noline_print("\n")


def reflect(coordinate: list[int], component: Axis, at: int):
    """Like a regular reflection.

    Difference: a *component*, like the X component, is reflected. That is to say,
    this function does not reflect *along an axis* (reflecting the X component requires
    folding along the Y axis.)
    """
    coordinate[component] -= 2 * (coordinate[component] - at)
    return coordinate


def fold(coordinate: list[int], component: Axis, at: int):
    """Folding is like reflecting, but one-sided."""
    if coordinate[component] > at:
        coordinate = reflect(coordinate, component, at)
    return coordinate


for i, (component, at) in enumerate(instructions):
    for dot in dots:
        dot = fold(dot, component, at)

    if i == 0:
        print(len(set(tuple(dot) for dot in dots)))

show(dots)
