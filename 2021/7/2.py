from collections import Counter


with open("input.txt") as f:
    positions = Counter(int(n) for n in f.readline().strip().split(","))


fuel_requirements = []


def fuel(distance):
    """Regular quadratic equation."""
    a = 0.5
    b = 0.5
    c = 0
    return a * distance ** 2 + b * distance + c


for position_to_check in range(max(positions) + 1):
    fuel_requirement = sum(
        fuel(abs(pos - position_to_check)) * n for pos, n in positions.items()
    )
    fuel_requirements.append(fuel_requirement)


print(min(fuel_requirements))
