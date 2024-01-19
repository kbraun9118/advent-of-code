import lib
import re

MARKER = re.compile(r"^\((\d+)x(\d+)\)")


def decompress(input: str) -> str:
    p = 0
    decompressed = ""
    while p < len(input):
        if input[p] != "(":  # )
            decompressed += input[p]
            p += 1
        else:
            match = MARKER.match(input[p:])
            if match is not None:
                length, repeat = match.groups()
                p += match.end() + match.start()
                next = input[p: p + int(length)]
                for _ in range(int(repeat)):
                    decompressed += next
                p += int(length)
    return decompressed


def decompress_iter(input: str) -> int:
    p = 0
    decompressed_len = 0
    while p < len(input):
        if input[p] != "(":  # )
            decompressed_len += 1
            p += 1
        else:
            match = MARKER.match(input[p:])
            if match is not None:
                length, repeat = match.groups()
                p += match.end() + match.start()
                decompressed_len += int(repeat) * decompress_iter(
                    input[p: p + int(length)]
                )
                p += int(length)
    return decompressed_len


def part_1(input: str) -> int:
    return len(decompress(input))


def part_2(input: str) -> int:
    return decompress_iter(input)


if __name__ == "__main__":
    input = lib.read_input_file("09")[0]
    lib.print_part_1(part_1(input))
    lib.print_part_2(part_2(input))
