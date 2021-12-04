from math import prod

coords = {
    "horizontal": 0,
    "depth": 0,
}

with open("input.txt") as f:
    for instruction in f:
        elements = instruction.split()
        direction = elements[0]
        magnitude = int(elements[1])

        if direction == "forward":
            coords["horizontal"] += magnitude
        elif direction == "up":
            coords["depth"] -= magnitude
        elif direction == "down":
            coords["depth"] += magnitude
        else:
            raise TypeError("Invalid direction.")

print(coords)
print(int(prod(coords.values())))
