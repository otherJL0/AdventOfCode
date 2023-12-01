from pathlib import Path


DIGITS = {
    "zero": "0",
    "one": "1",
    "two": "2",
    "three": "3",
    "four": "4",
    "five": "5",
    "six": "6",
    "seven": "7",
    "eight": "8",
    "nine": "9",
}


def read_input() -> list[str]:
    return (Path.cwd() / "input").read_text().splitlines()


def recover_calibration_value(code: str) -> int:
    digits = [x for x in code if x.isdigit()]
    num = int(digits[0] + digits[-1])
    return num


def recover2(code: str) -> int:
    digits: list[str] = []
    letters = ""
    for c in code:
        if c.isdigit():
            digits.append(c)
            continue
        letters += c
        for word, digit in DIGITS.items():
            if letters.endswith(word):
                digits.append(digit)
                break
    return int(digits[0] + digits[-1])


def part_1() -> int:
    codes = read_input()

    return sum(map(recover_calibration_value, codes))


def part_2() -> int:
    codes = read_input()
    return sum(map(recover2, codes))


if __name__ == "__main__":
    print(part_1())
    print(part_2())
