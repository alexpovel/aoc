from collections import Counter
from typing import Any

with open("input.txt") as f:
    numbers = [number.strip() for number in f.readlines()]


def pivot(rows: list[Any]):
    n_columns = len(rows[0])

    assert all(len(row) == n_columns for row in rows)

    columns: list[Any] = [[] for _ in range(n_columns)]

    for row in rows:
        for i, cell in enumerate(row):
            columns[i].append(cell)

    return columns


def most_common_element(iterable):
    item_index = 0
    count_index = 1

    most_commons = Counter(iterable).most_common()

    if (
        len(most_commons) > 1
        and most_commons[0][count_index] == most_commons[1][count_index]
    ):
        # A tie
        return None
    return most_commons[0][item_index]


def least_common_element(iterable):
    item_index = 0
    count_index = 1

    most_commons = Counter(iterable).most_common()

    if (
        len(most_commons) > 1
        and most_commons[-1][count_index] == most_commons[-2][count_index]
    ):
        # A tie
        return None
    return most_commons[-1][item_index]


o2_candidates = numbers
i = 0
while len(o2_candidates) > 1:
    current_column = pivot(o2_candidates)[i]
    o2_candidates = [
        c for c in o2_candidates if c[i] == (most_common_element(current_column) or "1")
    ]
    i += 1

co2_candidates = numbers
j = 0
while len(co2_candidates) > 1:
    current_column = pivot(co2_candidates)[j]
    co2_candidates = [
        c
        for c in co2_candidates
        if c[j] == (least_common_element(current_column) or "0")
    ]
    j += 1

print(int(o2_candidates[0], 2) * int(co2_candidates[0], 2))
