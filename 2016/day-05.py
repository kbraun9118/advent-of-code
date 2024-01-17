import lib
import hashlib


def part_1(input: str) -> str:
    out = ""
    i = 0
    while len(out) < 8:
        md5 = hashlib.md5(f"{input}{i}".encode()).hexdigest()
        if md5.startswith("00000"):
            # print(f"Found {i}")
            out = out + md5[5]
            # print(f"Out now {out}")
        i += 1
    return out


def part_2(input: str) -> str:
    out = [" ", " ", " ", " ", " ", " ", " ", " "]
    i = 0
    while " " in out:
        md5 = hashlib.md5(f"{input}{i}".encode()).hexdigest()
        if (
            md5.startswith("00000")
                and int(md5[5], 16) < 8
                and out[int(md5[5])] == " "
        ):
            out[int(md5[5])] = md5[6]
        i += 1
    return "".join(out)


if __name__ == "__main__":
    input = lib.read_input_file("05")[0]
    lib.print_part_1(part_1(input))
    lib.print_part_2(part_2(input))
