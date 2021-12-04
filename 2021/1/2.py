with open("input.txt") as f:
    depths = [int(depth) for depth in f.readlines()]

window_size = 3

previous_total = None

deltas = []

n = len(depths)

for left in range(n):
    right = left + window_size
    current_total = sum(depths[left:right])

    if previous_total is None:
        previous_total = current_total
        continue

    deltas.append(current_total - previous_total)
    if right == n:
        break
    previous_total = current_total

print(len(list(filter(lambda x: x > 0, deltas))))
