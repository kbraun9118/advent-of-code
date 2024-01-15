from typing import TypedDict
import lib

class Directions(TypedDict):
    dir: int
    dist: int


def part1(directions: Directions):

if __name__ == "__main__":
    lines = lib.read_input_file("01")[0].split(", ")
    directions = [{"dir": line[:1], "dist": int(line[1:])} for line in lines]
