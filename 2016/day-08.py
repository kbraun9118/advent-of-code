import lib

class Screen:
    def __init__(self) -> None:
        self.pixels: set[tuple[int, int]] = set()

    def rect(self, x: int, y: int): 

if __name__ == "__main__":
    lines = lib.read_test_file("08")
    for line in lines:
        print(line)
