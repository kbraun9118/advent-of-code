def read_input_file(day: str) -> list[str]:
    lines = open(f"../input/2016/day-{day}.txt", "r")
    return [line.strip() for line in lines.readlines() if line != "\n"]


def read_test_file(day: str) -> list[str]:
    lines = open(f"./test/day-{day}.txt", "r")
    return [line.strip() for line in lines.readlines() if line != "\n"]
