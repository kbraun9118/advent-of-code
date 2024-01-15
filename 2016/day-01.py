from typing import TypedDict
import lib


class Directions(TypedDict):
    dir: str
    dist: int


def part1(directions: list[Directions]) -> int:
    x, y = 0, 0
    facing = 0
    for direction in directions:
        dist = direction["dist"]
        match direction["dir"]:
            case "L":
                facing = (facing - 1) % 4
            case "R":
                facing = (facing + 1) % 4
        match facing:
            case 0:
                y -= dist
            case 1:
                x += dist
            case 2:
                y += dist
            case 3:
                x -= dist
    return abs(x) + abs(y)


def part2(directions: list[Directions]) -> int:
    x, y = 0, 0
    facing = 0
    visited: set[tuple[int, int]] = set()
    for direction in directions:
        dist = direction["dist"]
        match direction["dir"]:
            case "L":
                facing = (facing - 1) % 4
            case "R":
                facing = (facing + 1) % 4
        match facing:
            case 0:
                y -= dist
            case 1:
                x += dist
            case 2:
                y += dist
            case 3:
                x -= dist
        if (x, y) in visited:
            return abs(x) + abs(y)
        visited.add((x, y))
    return abs(x) + abs(y)


if __name__ == "__main__":
    # lines = lib.read_input_file("01")[0].split(", ")
    lines = "R8, R4, R4, R8".split(", ")
    directions: list[Directions] = [
        {"dir": line[:1], "dist": int(line[1:])} for line in lines
    ]
    lib.print_part_1(part1(directions))
    lib.print_part_2(part2(directions))
