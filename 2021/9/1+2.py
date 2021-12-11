import operator as op
from dataclasses import dataclass
from functools import cache, reduce
from typing import Callable


@dataclass(frozen=True)  # Need immutability for hashing
class Position:
    i: int
    j: int

    def __add__(self, other):
        klass = self.__class__
        return klass(self.i + other.i, self.j + other.j)


class Grid:
    def __init__(
        self, grid: list[list[int]], basins: list[set[Position]] = None
    ) -> None:
        self.rows = grid

        self._directions = [
            Position(0, 1),
            Position(0, -1),
            Position(-1, 0),
            Position(1, 0),
        ]

    def __iter__(self):
        return iter(self.rows)

    def __getitem__(self, key: Position):
        i = key.i
        j = key.j
        if i < 0 or j < 0:
            # Disable negative indexing wrap-around. If an element outside the grid is
            # requested, an error should be raised instead. This is probably terrible.
            raise IndexError
        return self.rows[i][j]

    @property
    def minima(self):
        return self._positions_with_all_valid_neighbors(criterion=op.gt)

    @property
    def maxima(self):
        return self._positions_with_all_valid_neighbors(criterion=op.lt)

    @cache
    def _positions_with_all_valid_neighbors(self, criterion: Callable):
        positions = set()

        for i, row in enumerate(self):
            for j, _ in enumerate(row):
                position = Position(i, j)
                if self._all_neighbors_valid(position=position, criterion=criterion):
                    positions.add(position)
        return positions

    @cache
    def _all_neighbors_valid(
        self,
        position: Position,
        criterion: Callable,
    ):
        return self._neighbors(position) == self._valid_neighbors(position, criterion)

    @cache
    def _neighbors(self, position: Position):
        neighbors = set()
        for direction in self._directions:
            neighbor_position = position + direction
            try:
                self[neighbor_position]
            except IndexError:
                pass
            else:
                neighbors.add(neighbor_position)
        return neighbors

    @cache
    def _valid_neighbors(self, position: Position, criterion: Callable):
        return {
            neighbor
            for neighbor in self._neighbors(position)
            if criterion(self[neighbor], self[position])
        }

    def branch_out(self, start: Position, criterion: Callable):
        positions = {start}

        for neighbor in self._valid_neighbors(position=start, criterion=criterion):
            positions.add(neighbor)
            positions |= self.branch_out(start=neighbor, criterion=criterion)

        return positions


with open("input.txt") as f:
    grid = Grid([[int(digit) for digit in number.strip()] for number in f.readlines()])


basins = [
    grid.branch_out(start=minimum, criterion=lambda neighbor, cell: cell < neighbor < 9)
    for minimum in grid.minima
]

print(sum(grid[pos] + 1 for pos in grid.minima))
print(reduce(op.mul, sorted(len(basin) for basin in basins)[-3:]))
