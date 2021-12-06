from collections import Counter

with open("input.txt") as f:
    fish = [int(n) for n in f.readline().strip().split(",")]


school = Counter(fish)

n_days = 256

initial_days_left = 8
regular_days_left = 6

totals = []

for n in range(n_days):
    for days_left in range(initial_days_left + 1):
        # Shift every age group one down:
        school[days_left - 1] += school[days_left]
        school[days_left] = 0

    # Reset the group of fish that went below 0, aka reset existing fish and
    # spawn a new batch of initial ones:
    school[initial_days_left] += school[-1]
    school[regular_days_left] += school[-1]
    school[-1] = 0
    totals.append(school.total())

print("Part 1:", totals[79])
print("Part 2:", totals[-1])
