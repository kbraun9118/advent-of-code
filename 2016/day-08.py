import lib

MAX_Y = 6
MAX_X = 50


class Screen:
    def __init__(self) -> None:
        self.pixels: set[tuple[int, int]] = set()

    def rect(self, x: int, y: int):
        for i in range(x):
            for j in range(y):
                self.pixels.add((i, j))

    def rotate_row(self, row_y: int, by: int):
        to_remove = []
        to_add = []
        for (x, y) in self.pixels:
            if y == row_y:
                to_remove.append((x, y))
                new_x = (x + by) % MAX_X
                to_add.append((new_x, y))
        for (x, y) in to_remove:
            self.pixels.remove((x, y))
        for (x, y) in to_add:
            self.pixels.add((x, y))

    def rotate_column(self, column_x: int, by: int):
        to_remove = []
        to_add = []
        for (x, y) in self.pixels:
            if x == column_x:
                to_remove.append((x, y))
                new_y = (y + by) % MAX_Y
                to_add.append((x, new_y))
        for (x, y) in to_remove:
            self.pixels.remove((x, y))
        for (x, y) in to_add:
            self.pixels.add((x, y))

    def print(self):
        for y in range(MAX_Y):
            for x in range(MAX_X):
                if (x, y) in self.pixels:
                    print("█", end="")
                else:
                    print(" ", end="")
            print()


def run_lines(lines: list[str]) -> Screen:
    screen = Screen()
    for line in lines:
        if line.startswith("rect"):
            dimensions = line.split(" ")[1]
            [x, y] = dimensions.split("x")
            screen.rect(int(x), int(y))
        if line.startswith("rotate column"):
            split = line.split(" ")
            x = int(split[2].split("=")[1])
            by = int(split[4])
            screen.rotate_column(x, by)
        if line.startswith("rotate row"):
            split = line.split(" ")
            y = int(split[2].split("=")[1])
            by = int(split[4])
            screen.rotate_row(y, by)
    return screen


if __name__ == "__main__":
    lines = lib.read_input_file("08")
    screen = run_lines(lines)
    lib.print_part_1(len(screen.pixels))
    lib.print_part_2("")
    screen.print()
