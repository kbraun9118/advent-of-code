import lib


def parse(lines: list[str]) -> list[list[int]]:
    return [[int(n) for n in line.split(" ") if n != ""] for line in lines]


def is_triangle(values: list[int]) -> bool:
    return (
        values[0] + values[1] > values[2]
        and values[0] + values[2] > values[1]
        and values[1] + values[2] > values[0]
    )


def part_1(input: list[list[int]]) -> int:
    count = 0
    for values in input:
        if is_triangle(values):
            count += 1
    return count


def part_2(input: list[list[int]]) -> int:
    count = 0
    for i in range(0, len(input), 3):
        if is_triangle([input[i][0], input[i + 1][0], input[i + 2][0]]):
            count += 1
        if is_triangle([input[i][1], input[i + 1][1], input[i + 2][1]]):
            count += 1
        if is_triangle([input[i][2], input[i + 1][2], input[i + 2][2]]):
            count += 1
    return count


if __name__ == "__main__":
    lines = lib.read_input_file("03")
    parsed = parse(lines)
    lib.print_part_1(part_1(parsed))
    lib.print_part_2(part_2(parsed))
