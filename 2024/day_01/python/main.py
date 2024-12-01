from pathlib import Path
import re
from functools import reduce
from collections import Counter


def read_input() -> list[str]:
    return [line for line in Path("input").read_text().split("\n") if len(line)]


def input_to_lists(pairs: list[str]) -> tuple[list[int], list[int]]:
    left_list: list[int] = []
    right_list: list[int] = []
    whitespace = re.compile(r"\s+")
    for nums in pairs:
        left, right = [int(num) for num in whitespace.split(nums)]
        left_list.append(left)
        right_list.append(right)
    return sorted(left_list), sorted(right_list)


def part_one() -> int:
    left, right = input_to_lists(read_input())
    return reduce(lambda acc, pair: acc + abs(pair[0] - pair[1]), zip(left, right), 0)


def part_two() -> int:
    left, right = input_to_lists(read_input())
    counter = Counter(right)
    return reduce(lambda acc, num: acc + num * counter.get(num, 0), left, 0)


if __name__ == "__main__":
    print(f"{part_one()=}")
    print(f"{part_two()=}")
