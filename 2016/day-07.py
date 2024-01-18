import lib
import re


class Ipv4:
    def __init__(self, input: str) -> None:
        self.address = input
        hypernet = re.compile(r"\[\w+\]")
        search = hypernet.search(input)
        hypernets = []
        nets = []
        rest = input
        while search is not None:
            nets.append(rest[:search.start()])
            hypernets.append(rest[search.start() + 1: search.end() - 1])
            rest = rest[search.end():]
            search = hypernet.search(rest)
        nets.append(rest)
        self.nets: list[str] = nets
        self.hypernets: list[str] = hypernets

    def is_tls(self) -> bool:
        for hypernet in self.hypernets:
            for i in range(len(hypernet) - 3):
                if (
                        hypernet[i] != hypernet[i + 1]
                        and hypernet[i] == hypernet[i + 3]
                        and hypernet[i + 1] == hypernet[i + 2]
                ):
                    return False
        for net in self.nets:
            for i in range(len(net) - 3):
                if (
                        net[i] != net[i + 1]
                        and net[i] == net[i + 3]
                        and net[i + 1] == net[i + 2]
                ):
                    return True
        return False

    def is_ssl(self) -> bool:
        for net in self.nets:
            for i in range(len(net) - 2):
                if net[i] == net[i + 2] and net[i] != net[i + 1]:
                    bab = net[i + 1] + net[i] + net[i + 1]
                    if any([bab in hypernet for hypernet in self.hypernets]):
                        return True
        return False


def part_1(addresses: list[Ipv4]) -> int:
    return len([a for a in addresses if a.is_tls()])


def part_2(addresses: list[Ipv4]) -> int:
    return len([a for a in addresses if a.is_ssl()])


if __name__ == "__main__":
    lines = lib.read_input_file("07")
    addresses = [Ipv4(address) for address in lines]
    lib.print_part_1(part_1(addresses))
    lib.print_part_2(part_2(addresses))
