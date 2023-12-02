from pathlib import Path
from typing import TypedDict


class GameRound(TypedDict):
    red: int
    blue: int
    green: int


Game = list[GameRound]


def read_input() -> list[str]:
    return [line.strip() for line in Path("input").read_text().splitlines()]


def parse_game(line: str) -> list[GameRound]:
    _, rounds = line.split(":")
    game: list[GameRound] = []
    for round in rounds.split(";"):
        game.append(GameRound(red=0, blue=0, green=0))
        for pulled in round.split(","):
            count, color = (x.strip() for x in pulled.strip().split(" "))
            game[-1][color] += int(count)
    return game


def part_one(games: list[Game]) -> int:
    bag = {"red": 12, "blue": 14, "green": 13}
    valid_games = 0
    for id, game in enumerate(games):
        for color, max_count in bag.items():
            if max(round[color] for round in game) > max_count:
                break
        else:
            valid_games += id + 1
    return valid_games


def part_two(games: list[Game]) -> int:
    result = 0
    for game in games:
        power = 1
        for color in ("red", "blue", "green"):
            power *= max(round[color] for round in game)
        result += power
    return result


if __name__ == "__main__":
    games: list[Game] = [parse_game(line) for line in read_input()]
    print(part_one(games))
    print(part_two(games))
