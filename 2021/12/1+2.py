from collections import defaultdict
from typing import Callable

with open("input.txt") as f:
    graph = defaultdict(set)
    for line in f:
        connections = line.strip().split("-")
        for vertex in connections:
            for other_vertex in connections:
                if vertex == other_vertex:
                    continue
                graph[vertex].add(other_vertex)


def part1_predicate(vertex, path, start, target):
    return vertex.isupper() or vertex not in path


def part2_predicate(vertex, path, start, target):
    base_predicate = part1_predicate(vertex, path, start, target)
    lowers = [vertex for vertex in path if vertex.islower()]
    dupefree = len(set(lowers)) == len(lowers)
    return base_predicate or (dupefree and vertex not in [start, target])


def depth_first_search(
    graph: dict[str, set[str]], start: str, target: str, predicate: Callable
):
    stack = [[start]]  # Stack of potential paths

    while stack:
        path = stack.pop()  # Examine this one
        vertex = path[-1]

        if vertex == target:
            yield path
            continue

        for neighbor in graph[vertex]:
            if predicate(neighbor, path, start, target):
                # Put this back on for further examination in the next iteration, since
                # this neighbor passed.
                stack.append(path + [neighbor])
            # Predicate wasn't fulfilled, hence path was popped of stack without
            # putting it back on: it was invalid and is gone.


print(len(list(depth_first_search(graph, "start", "end", part1_predicate))))
print(len(list(depth_first_search(graph, "start", "end", part2_predicate))))
