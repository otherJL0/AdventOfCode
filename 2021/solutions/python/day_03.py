import sys
from collections import Counter
from typing import List

from aoc import get_input_path, read_input


def part_one(lines: List[str]):
    transposed: List[str] = []
    for line in lines:
        for j, bit in enumerate(line):
            try:
                transposed[j] += bit
            except IndexError:
                transposed.append("")
    gamma_rate = "0b" + "".join(
        [Counter(pos).most_common(2)[0][0] for pos in transposed]
    )
    epsilon_rate = "0b" + "".join(
        [Counter(pos).most_common(2)[1][0] for pos in transposed]
    )
    print(gamma_rate)
    print(epsilon_rate)
    return int(gamma_rate, 2) * int(epsilon_rate, 2)


def part_two(lines: List[str]):
    pass


def main():
    day: str = sys.argv[0].rstrip(".py").lstrip("day_")
    lines: List[str] = read_input(get_input_path(day))
    result = part_one(lines)
    print(result)


if __name__ == "__main__":
    main()
