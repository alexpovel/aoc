from functools import reduce

observations = []
with open("input.txt") as f:
    for line in f.readlines():
        left, right = line.split("|")
        observations.append(
            (
                # We rely on hashability aka immutability, hence frozenset over set.
                # We don't care for duplicates, hence a set is possible.
                {frozenset(signal) for signal in left.split()},
                # This cannot be a set: we have to preserve the original order and
                # duplicates are possible.
                [frozenset(output) for output in right.split()],
            )
        )

# The below is mostly just set logic applied to the following mapping.
# For example, if we add letter 'd' to digit '0', we get digit '8'.
# If we substract the set behind digit '1' from the digit '4' set, we get the set of
# letters 'b' and 'd', and so forth.
# For all this, we don't need to know the new/mangled mappings, we can simply ignore them.

#   0:      1:      2:      3:      4:
#  aaaa    ....    aaaa    aaaa    ....
# b    c  .    c  .    c  .    c  b    c
# b    c  .    c  .    c  .    c  b    c
#  ....    ....    dddd    dddd    dddd
# e    f  .    f  e    .  .    f  .    f
# e    f  .    f  e    .  .    f  .    f
#  gggg    ....    gggg    gggg    ....

#   5:      6:      7:      8:      9:
#  aaaa    aaaa    aaaa    aaaa    aaaa
# b    .  b    .  .    c  b    c  b    c
# b    .  b    .  .    c  b    c  b    c
#  dddd    dddd    ....    dddd    dddd
# .    f  e    f  .    f  e    f  .    f
# .    f  e    f  .    f  e    f  .    f
#  gggg    gggg    ....    gggg    gggg

part1 = 0
part2 = 0

for observation in observations:
    observered_signals, observed_outputs = observation

    digits = {}
    letters = {}

    for signal in observered_signals:
        length = len(signal)
        # The signals are designed such that we encounter all of these guaranteed in
        # each observation, so that we can rely on knowing how 1, 4, 7 and 8
        # are constructed.
        if length == 2:
            digits[1] = signal
        elif length == 4:
            digits[4] = signal
        elif length == 3:
            digits[7] = signal
        elif length == 7:
            digits[8] = signal

    letters["a"] = digits[7] - digits[1]

    for signal in observered_signals:
        if signal == digits[8]:
            continue
        if digits[4] | letters["a"] < signal:
            # Adding segment 'a' to the '4' digit yields a set that only has 8 and 9 as
            # supersets. Having excluded 8, we know we found 9.
            digits[9] = signal
            break

    letters["e"] = digits[8] - digits[9]

    for signal in observered_signals:
        if signal == digits[8]:
            continue
        if signal | digits[1] == digits[8]:
            digits[6] = signal
            break

    digits[5] = digits[6] - letters["e"]

    for signal in observered_signals:
        if signal == digits[8]:
            continue
        if signal | (digits[4] - digits[1]) == digits[8]:
            digits[0] = signal
            break

    # The below relies on (or at least is *much* more concise to write) filtering out
    # all processed values first, so that only the last two unknowns remain.
    observered_signals -= set(digits.values())

    for signal in observered_signals:
        if digits[1] < signal:
            digits[3] = signal
        else:
            digits[2] = signal

    # Between sets of segments and digits, we have a bijective mapping, hence inverting
    # the mapping is possible at no loss of information.
    signals = {v: k for k, v in digits.items()}

    output_digits = [signals[o] for o in observed_outputs]

    for digit in output_digits:
        part1 += digit in [1, 4, 7, 8]

    part2 += reduce(lambda x, y: x * 10 + y, output_digits)

print(part1)
print(part2)
