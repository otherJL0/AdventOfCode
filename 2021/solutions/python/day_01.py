import collections
import itertools as it
import sys
from typing import List

from aoc import get_input_path, read_input


def transform(line: str) -> int:
    return int(line)


def sliding_window(lines: List[int], n: int):
    iterable = iter(lines)
    window = collections.deque(it.islice(iterable, n), maxlen=n)
    if len(window) == n:
        yield tuple(window)
    for item in iterable:
        window.append(item)
        yield tuple(window)


def count_increasing_depths(depths: List[int]) -> int:
    count = 0
    for window in sliding_window(depths, 2):
        if window[1] > window[0]:
            count += 1
    return count


def count_increasing_avg_depth(depths: List[int]):
    count = 0
    prev_sum = sum(depths[:3])
    for window in sliding_window(depths, 3):
        current_sum = sum(window)
        if current_sum > prev_sum:
            count += 1
        prev_sum = current_sum
    return count


def main():
    day: str = sys.argv[0].rstrip(".py").lstrip("day_")
    lines: List[str] = read_input(get_input_path(day))
    formatted_input: List[int] = list(map(transform, lines))
    count = count_increasing_depths(formatted_input)
    print(count)


if __name__ == "__main__":
    main()
