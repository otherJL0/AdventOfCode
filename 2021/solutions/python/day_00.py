import sys

from aoc import get_input_path, read_input


def main():
    day = sys.argv[0].rstrip(".py").lstrip("day_")
    lines = read_input(get_input_path(day))
    print(lines)


if __name__ == "__main__":
    main()
