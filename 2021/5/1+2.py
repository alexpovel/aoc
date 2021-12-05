"""Script reads from stdin. Requires `pip install rich`, just for fun.

Script takes c. 90 seconds to finish, which is terrible.
"""


import fileinput
from dataclasses import dataclass
from typing import Iterable

from rich.progress import track


@dataclass
class Coordinate:
    x: int
    y: int
    value: int = 0


class Line:
    def __init__(self, begin: Coordinate, end: Coordinate):
        coords = [begin, end]

        self.x_max = max(coord.x for coord in coords)
        self.x_min = min(coord.x for coord in coords)
        self.y_max = max(coord.y for coord in coords)
        self.y_min = min(coord.y for coord in coords)

        self.horizontal = begin.y == end.y
        self.vertical = begin.x == end.x

        self.slope = (
            NotImplemented
            if self.vertical
            else (end.y - begin.y) / (end.x - begin.x)
        )

        self.shift = (
            NotImplemented if self.vertical else end.y - self.slope * end.x
        )

    def __contains__(self, coordinate: Coordinate):
        within_vertical_bounds = self.y_min <= coordinate.y <= self.y_max
        within_horizontal_bounds = self.x_min <= coordinate.x <= self.x_max

        within_bounds = within_vertical_bounds and within_horizontal_bounds

        if not within_bounds:
            return False
        if self.vertical:
            return True

        fulfills_linear_equation = (
            coordinate.y == self.slope * coordinate.x + self.shift
        )
        return fulfills_linear_equation


class Grid:
    def __init__(self, lines: Iterable[Line]) -> None:
        self.x_max = 0
        self.y_max = 0

        self.n_dangerous_areas = 0

        for line in lines:
            self.x_max = max(self.x_max, line.x_max)
            self.y_max = max(self.y_max, line.y_max)

        self._grid = [
            [Coordinate(x, y) for x in range(self.x_max + 1)]
            for y in range(self.y_max + 1)
        ]

        for row in track(self):
            for _, cell in enumerate(row):
                for line in lines:
                    cell.value += cell in line
                self.n_dangerous_areas += cell.value >= 2

    def __str__(self) -> str:
        formatted_rows = []
        for row in self:
            formatted_rows.append(
                "\t".join(str(cell.value) if cell.value else "." for cell in row)
            )
        return "\n".join(formatted_rows)

    def __iter__(self):
        yield from self._grid

    def __len__(self):
        return len(self._grid)


def get_data():
    lines = []
    for inputline in fileinput.input():
        inputline = inputline.replace("-", "").replace(">", "")
        line = inputline.split()
        begin = Coordinate(*[int(number) for number in line[0].split(",")])
        end = Coordinate(*[int(number) for number in line[1].split(",")])
        lines.append(Line(begin, end))
    return lines



# lines = list(filter(lambda line: line.horizontal or line.vertical, get_data()))  # Part 1
lines = get_data()  # Part 2

grid = Grid(lines)

# print(grid)
print(grid.n_dangerous_areas)
