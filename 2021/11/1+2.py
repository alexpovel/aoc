from itertools import product


with open("input.txt") as f:
    grid = [[int(digit) for digit in row.strip()] for row in f.readlines()]

adjacents = list(product((0, -1, 1), repeat=2))  # All possible directions to move to

adjacents.remove((0, 0))


N_FLASHES = 0


def increment(grid):
    for i, row in enumerate(grid):
        for j, _ in enumerate(row):
            grid[i][j] += 1
    return grid


def allzero(grid):
    # Could be pushed into `increment` so we don't do superfluous looping here, but
    # separation of concerns or whatever
    for i, row in enumerate(grid):
        for j, _ in enumerate(row):
            if grid[i][j] != 0:
                return False
    return True


def trigger(grid):
    global N_FLASHES  # The way I maintain this state makes the US seem well-run

    for i, row in enumerate(grid):
        for j, _ in enumerate(row):
            if grid[i][j] > 9:
                N_FLASHES += 1
                # Grid of integers: the latter are immutable, hence we cannot have
                # references/pointers to them and have to index into `grid` repeatedly
                grid[i][j] = 0
                for di, dj in adjacents:
                    neighbor_i = i + di
                    neighbor_j = j + dj
                    try:
                        if neighbor_i < 0 or neighbor_j < 0:
                            # Disable list wrap-around through negative slices. This is kind of
                            # pathetic. Time to learn numpy.
                            raise IndexError
                        if grid[neighbor_i][neighbor_j] > 0:
                            grid[neighbor_i][neighbor_j] += 1
                    except IndexError:
                        continue
                grid = trigger(grid)
    return grid


def step(grid):
    return trigger(increment(grid))


n_step = 0

while not allzero(grid):
    n_step += 1
    grid = step(grid)
    if n_step == 100:  # Breaks if allzero occurs before this step
        print(N_FLASHES)

print(n_step)
