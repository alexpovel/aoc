"""Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".

Equipped with this new information, you now need to find the real first and last digit on each line. For example:

two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen

In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.

What is the sum of all of the calibration values?
"""

import re
from pathlib import Path

with open(Path(__file__).parent / "input_01.txt") as f:
    lines = f.readlines()

digits = {
    "one": 1,
    "two": 2,
    "three": 3,
    "four": 4,
    "five": 5,
    "six": 6,
    "seven": 7,
    "eight": 8,
    "nine": 9,
}

# Separate patterns for easier reasoning
first_p = re.compile(rf"^.*?(\d|{'|'.join(digits)})")
last_p = re.compile(rf".*(\d|{'|'.join(digits)}).*?$")

sum_ = 0
for line in lines:
    first_match = first_p.match(line)
    assert first_match is not None, f"No match found in line: {line}"

    first_raw = first_match.group(1)
    assert isinstance(first_raw, str), f"No first found in line: {line}"

    try:
        first = int(first_raw)
    except ValueError:
        first = digits[first_raw]

    last_match = last_p.match(line)
    if last_match is None:
        last = first
    else:
        last_raw = last_match.group(1)
        try:
            last = int(last_raw)
        except ValueError:
            last = digits[last_raw]

    sum_ += first * 10 + last

assert sum_ == 54885
print(sum_)
