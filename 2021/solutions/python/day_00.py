import sys
from typing import List

from aoc import get_input_path, read_input


def transform(line: str) -> str:
    return line


def part_one(lines: List[str]):
    pass


def part_two(lines: List[str]):
    pass


def main():
    day: str = sys.argv[0].rstrip(".py").lstrip("day_")
    lines: List[str] = read_input(get_input_path(day))
    formatted_input: List[str] = [transform(line) for line in lines]
    print(formatted_input)


if __name__ == "__main__":
    main()
