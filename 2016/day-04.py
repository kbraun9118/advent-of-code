import lib


class Room:
    def __init__(self, line: str):
        split = line.split("-")
        self.name = "-".join(split[:-1])
        self.checksum = split[-1][4:-1]
        self.id = int(split[-1][0:3])

    def is_real(self) -> bool:
        occurances = {
            item: self.name.count(item)
            for item
            in "".join(self.name.replace("-", ''))
        }
        occurance_values = list(occurances.items())
        occurance_values.sort(key=lambda v: (
            v[1], -1 * ord(v[0])), reverse=True)
        hashed = "".join([value[0] for value in occurance_values])[0:5]
        return self.checksum == hashed


    def decrypt(self) -> str:
        "".join([])


def part_1(lines: list[str]) -> int:
    rooms = [Room(line) for line in lines]
    real = [room.id for room in rooms if room.is_real()]
    return sum(real)


if __name__ == "__main__":
    lines = lib.read_input_file("04")
    lib.print_part_1(part_1(lines))
