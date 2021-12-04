coords = {
    "horizontal": 0,
    "depth": 0,
    "aim": 0,
}

with open("input.txt") as f:
    for instruction in f:
        elements = instruction.split()
        direction = elements[0]
        magnitude = int(elements[1])

        if direction == "forward":
            coords["horizontal"] += magnitude
            coords["depth"] += coords["aim"] * magnitude
        elif direction == "up":
            coords["aim"] -= magnitude
        elif direction == "down":
            coords["aim"] += magnitude
        else:
            raise TypeError("Invalid direction.")

print(coords)
print(coords["horizontal"] * coords["depth"])
