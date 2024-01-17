def read_input_file(day: str) -> list[str]:
    lines = open(f"../input/2016/{day}.txt", "r")
    return [line.strip() for line in lines.readlines() if line != "\n"]


def read_test_file(day: str) -> list[str]:
    lines = open(f"./test/{day}.txt", "r")
    return [line.strip() for line in lines.readlines() if line != "\n"]


def print_part_1(ans):
    print(f"Part 1: {ans}")


def print_part_2(ans):
    print(f"Part 2: {ans}")
