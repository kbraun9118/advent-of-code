import lib

KEYPAD_1 = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9],
]

MAX_W_1 = 2

KEYPAD_2 = [
    [" ", " ", "1", " ", " "],
    [" ", "2", "3", "4", " "],
    ["5", "6", "7", "8", "9"],
    [" ", "A", "B", "C", " "],
    [" ", " ", "D", " ", " "],
]


MAX_W_2 = 4


def part_1(lines: list[str]) -> str:
    code = []
    x, y = 1, 1
    for line in lines:
        for ch in line:
            match ch:
                case "U":
                    y = max(0, y - 1)
                case "D":
                    y = min(MAX_W_1, y + 1)
                case "L":
                    x = max(0, x - 1)
                case "R":
                    x = min(MAX_W_1, x + 1)
        code.append(str(KEYPAD_1[y][x]))
    return "".join(code)


def part_2(lines: list[str]) -> str:
    code = []
    x, y = 0, 2
    for line in lines:
        for ch in line:
            cur_x, cur_y = x, y
            match ch:
                case "U":
                    y = max(0, y - 1)
                case "D":
                    y = min(MAX_W_2, y + 1)
                case "L":
                    x = max(0, x - 1)
                case "R":
                    x = min(MAX_W_2, x + 1)
            if KEYPAD_2[y][x] == " ":
                x, y = cur_x, cur_y
        code.append(KEYPAD_2[y][x])
    return "".join(code)


if __name__ == "__main__":
    lines = lib.read_input_file("02")
    lib.print_part_1(part_1(lines))
    lib.print_part_2(part_2(lines))
