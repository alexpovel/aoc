from dataclasses import dataclass
from typing import Iterable


@dataclass
class Cell:
    """A bingo field cell."""

    number: int
    marked: bool = False
    winner: bool = False


class Board:
    def __init__(self, board: Iterable, size: int) -> None:
        self._board = [Cell(int(number)) for number in board]
        self._size = size
        self._total = size ** 2
        self._winning_number = None
        self._winners = []

    def mark(self, number):
        for item in self:
            if item.number == number:
                item.marked = True

        self._update_winners()

        if self.won:
            self._winning_number = number

    def __str__(self) -> str:
        formatted_rows = []

        red = "\033[91m"
        bold = "\033[1m"
        end = "\033[0m"
        underline = "\033[4m"
        highlight_low = lambda string: red + bold + string + end
        highlight_high = lambda string: underline + highlight_low(string) + end

        for row in self.rows:
            new_row = []
            for cell in row:
                number = str(cell.number)
                if cell.winner:
                    new_row.append(highlight_high(number))
                elif cell.marked:
                    new_row.append(highlight_low(number))
                else:
                    new_row.append(number)
            formatted_rows.append("\t".join(new_row))
        return "\n".join(formatted_rows)

    def __getitem__(self, key):
        """Enables iterator protocol for `self`."""
        return self._board[key]

    @property
    def score(self):
        if self.won:
            return (
                sum(item.number for item in self if not item.marked)
                * self._winning_number
            )

    @property
    def rows(self):
        start = 0
        while start < self._total:
            end = start + self._size
            yield self[start:end]
            start += self._size

    @property
    def columns(self):
        for j in range(self._size):
            yield [self[j + i * self._size] for i in range(self._size)]

    def _update_winners(self):

        for row in self.rows:
            if all(cell.marked for cell in row):
                self._winners.extend(row)

        for column in self.columns:
            if all(cell.marked for cell in column):
                self._winners.extend(column)

        for winner in self._winners:
            winner.winner = True

    @property
    def won(self):
        return any(self._winners)



def get_input():
    boards = []
    with open("input.txt") as f:
        draws = [int(item) for item in next(f).split(",")]

        new_board = []
        for line in f:
            line = line.strip()
            if line:  # Non-empty line
                new_board.extend(int(number) for number in line.split())
            else:  # Empty line
                if new_board:  # Skip initial empties
                    boards.append(Board(new_board, size=5))
                    new_board = []
    return draws, boards

draws, boards = get_input()

winners = []

for draw in draws:
    for board in boards:
        board.mark(draw)
        if board.won and board not in winners:
            winners.append(board)

print("First winner:")
print(winners[0])
print(winners[0].score)
print("Last winner:")
print(winners[-1])
print(winners[-1].score)
