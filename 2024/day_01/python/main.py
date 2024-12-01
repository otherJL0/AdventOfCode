from pathlib import Path
import re
from functools import reduce
from collections import Counter


def read_input() -> list[str]:
    return [line for line in Path("../input").read_text().split("\n") if len(line)]


def input_to_lists(pairs: list[str]) -> tuple[list[int], list[int]]:
    left_list: list[int] = []
    right_list: list[int] = []
    whitespace = re.compile(r"\s+")
    for nums in pairs:
        left, right = [int(num) for num in whitespace.split(nums)]
        left_list.append(left)
        right_list.append(right)
    return sorted(left_list), sorted(right_list)


def part_one(left: list[int], right: list[int]) -> int:
    return reduce(lambda acc, pair: acc + abs(pair[0] - pair[1]), zip(left, right), 0)


def part_two(left: list[int], right: list[int]) -> int:
    counter = Counter(right)
    return reduce(lambda acc, num: acc + num * counter.get(num, 0), left, 0)


def main():
    left, right = input_to_lists(read_input())
    result_one = part_one(left, right)
    print(f"{result_one=}")
    result_two = part_two(left, right)
    print(f"{result_two=}")


if __name__ == "__main__":
    main()
