"""Made this work with `mypy --strict` for fun."""

from collections import Counter, deque
from typing import Any, Iterable, Sequence

with open("input.txt") as f:
    template = list(next(f).strip())
    next(f)  # Skip blank
    insertions: dict[tuple[str, ...], str] = {}
    for line in f:
        input_pair, _arrow, insertion = line.split()
        insertions[tuple(input_pair)] = insertion


def sliding_window(sequence: Sequence[Any], size: int) -> Iterable[tuple[Any, ...]]:
    initial, tail = sequence[:size], sequence[size:]
    window = deque(initial, maxlen=size)

    yield tuple(window)
    for element in tail:
        window.append(element)
        yield tuple(window)


pairs = Counter(pairs for pairs in sliding_window(template, 2))
single_elements = Counter(template)

for i in range(40):
    update: Counter[tuple[str, ...]] = Counter()
    for pair, count in pairs.items():
        left, right = pair
        try:
            insertion = insertions[pair]
        except KeyError:
            continue

        update[pair] -= count
        update[(left, insertion)] += count
        update[(insertion, right)] += count

        single_elements[insertion] += count

    pairs.update(update)

    if i == 9 or i == 39:
        most_common = single_elements.most_common()
        print(most_common[0][1] - most_common[-1][1])
