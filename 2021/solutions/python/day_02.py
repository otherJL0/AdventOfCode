import sys
from typing import List, Tuple

from aoc import get_input_path, read_input


def calculate_final_position(lines: List[str]) -> List[int]:
    position = [0,0]
    for _line in lines:
        line = _line.split()
        movement: Tuple[str, int] = (line[0], int(line[1]))
        match movement:
            case ["forward", magnitude]:
                position[0] += magnitude
            case ["down", magnitude]:
                position[1] += magnitude
            case ["up", magnitude]:
                position[1] -= magnitude
    return position


def calculate_final_position_with_aim(lines: List[str]) -> List[int]:
    aim = 0
    position = [0,0]
    for _line in lines:
        line = _line.split()
        movement: Tuple[str, int] = (line[0], int(line[1]))
        match movement:
            case ["forward", magnitude]:
                position[0] += magnitude
                position[1] += magnitude*aim
            case ["down", magnitude]:
                aim += magnitude
            case ["up", magnitude]:
                aim -= magnitude
    return (position)


def main():
    day = sys.argv[0].rstrip(".py").lstrip("day_")
    lines = read_input(get_input_path(day))
    x, y = calculate_final_position(lines)
    print(x, y)
    print(x * y)


if __name__ == "__main__":
    main()
