import lib

def part_1(lines: list[str]) -> str:
    out = ""
    for i in range(len(lines[0])):
        chs = [line[i] for line in lines]
        occurances = {ch: chs.count(ch) for ch in chs}
        out += max(occurances.items(), key=lambda v: v[1])[0]
    return out


def part_2(lines: list[str]) -> str:
    out = ""
    for i in range(len(lines[0])):
        chs = [line[i] for line in lines]
        occurances = {ch: chs.count(ch) for ch in chs}
        out += min(occurances.items(), key=lambda v: v[1])[0]
    return out


if __name__ == "__main__":
    lines = lib.read_input_file("06")
    lib.print_part_1(part_1(lines))
    lib.print_part_2(part_2(lines))
