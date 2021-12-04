with open("input.txt") as f:
    depths = [int(depth) for depth in f.readlines()]

previous = None

deltas = []

for depth in depths:
    if previous is None:
        previous = depth
        continue

    deltas.append(depth - previous)

    previous = depth

print(len(list(filter(lambda x: x > 0, deltas))))
