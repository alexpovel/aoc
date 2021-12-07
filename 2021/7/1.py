from collections import Counter


with open("input.txt") as f:
    positions = Counter(int(n) for n in f.readline().strip().split(","))


fuel_requirements = []

for position_to_check in range(max(positions) + 1):
    fuel_requirement = sum(
        abs(pos - position_to_check) * n for pos, n in positions.items()
    )
    fuel_requirements.append(fuel_requirement)


print(min(fuel_requirements))
