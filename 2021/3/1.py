from collections import defaultdict

with open("input.txt") as f:
    numbers = [number.strip() for number in f.readlines()]

tilts = defaultdict(int)

for number in numbers:
    for i, digit in enumerate(number):
        tilts[i] += 1 if digit == "1" else -1


gamma_rate = int("".join(str(int(tilt > 0)) for tilt in tilts.values()), base=2)
epsilon_rate = int("".join(str(int(tilt < 0)) for tilt in tilts.values()), base=2)

print(epsilon_rate * gamma_rate)
