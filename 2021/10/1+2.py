# from functools import reduce
from statistics import median

with open("input.txt") as f:
    lines = [line.strip() for line in f.readlines()]

pairs = {
    "(": ")",
    "[": "]",
    "{": "}",
    "<": ">",
}

points = {
    1: {  # Part 1
        ")": 3,
        "]": 57,
        "}": 1197,
        ">": 25137,
    },
    2: {  # Part 2
        ")": 1,
        "]": 2,
        "}": 3,
        ">": 4,
    },
}


syntax_error_score = 0
autocompletion_scores = []


for line in lines:
    stack = []

    for character in line:
        if character in pairs:
            stack.append(character)
        else:
            opening = stack.pop()
            closing = pairs[opening]
            if character != closing:
                syntax_error_score += points[1][character]
                break
    else:  # nobreak
        autocompletion_score = 0
        completion = (pairs[opening] for opening in stack[::-1])
        for character in completion:
            autocompletion_score *= 5
            autocompletion_score += points[2][character]
        autocompletion_scores.append(autocompletion_score)


print(syntax_error_score)
print(median(autocompletion_scores))
