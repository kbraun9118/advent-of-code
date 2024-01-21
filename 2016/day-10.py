from enum import Enum
import lib


class Type(Enum):
    Bot = 0
    Output = 1


class Bot:
    def __init__(
            self,
            id: int,
            high: tuple[Type, int],
            low: tuple[Type, int]
    ) -> None:
        self.id = id
        self.high = high
        self.low = low
        self.holding: list[int] = []
        self.handled: list[int] = []

    def give(self, num: int):
        self.holding.append(num)
        self.handled.append(num)

    def take(self) -> tuple[int, int]:
        high, low = (max(self.holding), min(self.holding))
        self.holding.clear()
        return (high, low)

    def is_full(self) -> bool:
        return len(self.holding) == 2


def parse_lines(
        lines: list[str]
) -> tuple[dict[int, Bot], dict[int, list[int]], list[tuple[int, int]]]:
    bots = dict()
    outputs = dict()
    instructions: list[tuple[int, int]] = list()
    for instruction in [i for i in lines if i.startswith("value")]:
        split = instruction.split(" ")
        instructions.append((int(split[1]), int(split[-1])))
    for definition in [d for d in lines if d.startswith("bot")]:
        split = definition.split(" ")
        id = int(split[1])
        low_type = Type.Bot if split[5] == "bot" else Type.Output
        low_id = int(split[6])
        high_type = Type.Bot if split[10] == "bot" else Type.Output
        high_id = int(split[11])
        if low_type == Type.Output and low_id not in outputs:
            outputs[low_id] = []
        if high_type == Type.Output and high_id not in outputs:
            outputs[high_id] = []
        bots[id] = Bot(id, (high_type, high_id), (low_type, low_id))
    return bots, outputs, instructions


def give_num(
        value: int,
        to: int,
        bots: dict[int, Bot],
        outputs: dict[int, list[int]]
):
    bots[to].give(value)
    while any([isf.is_full() for isf in bots.values()]):
        for bot in [bot for bot in bots.values() if bot.is_full()]:
            high, low = bot.take()
            if bot.high[0] == Type.Output:
                outputs[bot.high[1]].append(high)
            else:
                bots[bot.high[1]].give(high)
            if bot.low[0] == Type.Output:
                outputs[bot.low[1]].append(low)
            else:
                bots[bot.low[1]].give(low)


def part_1(bots: dict[int, Bot]) -> int:
    for (id, bot) in bots.items():
        if 17 in bot.handled and 61 in bot.handled:
            return id
    return -1


def part_2(outputs: dict[int, list[int]]):
    return outputs[0][0] * outputs[1][0] * outputs[2][0]


if __name__ == "__main__":
    lines = lib.read_input_file("10")
    bots, outputs, instructions = parse_lines(lines)
    for (value, to) in instructions:
        give_num(value, to, bots, outputs)
    lib.print_part_1(part_1(bots))
    lib.print_part_2(part_2(outputs))
