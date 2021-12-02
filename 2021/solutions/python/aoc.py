from pathlib import Path
from typing import List


def get_input_path(day: str) -> Path:
    return Path.home() / f"gh/otherJL0/AdventOfCode/2021/input/day_{day}/input"


def read_input(path: Path) -> List[str]:
    with open(path, "r") as input_file:
        return [line.strip() for line in input_file.readlines()]
