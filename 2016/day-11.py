from enum import Enum
import re
import lib
import copy
from typing import Self

COMPONENTS_REGEX = re.compile(r"(a (\w+)(( generator)|(-compatible microchip)))")

class Type(Enum):
    Generator = 0
    Microchip = 1

    
class Component:
    def __init__(self, type: Type, value: str) -> None:
        self.type = type
        self.value = value

    def __str__(self) -> str:
        return f"Component(type={self.type}, value={self.value})"
    
class Floors:
    def __init__(self, lines: list[str]) -> None:
        floors: dict[int, list[Component]] = dict()
        self.elevator = 1
        for (floor, line) in enumerate(lines):
           [_, components] = line.split("contains ")
           if components.startswith("nothing relevant"):
               floors[floor + 1] = []
           else:
               floor = floor + 1
               floors[floor] = []
               groups = COMPONENTS_REGEX.findall(line)
               for group in groups:
                   value = group[1]
                   type = group[2]
                   if type == " generator":
                       type = Type.Generator
                   else:
                       type = Type.Microchip
                   floors[floor].append(Component(type, value))
        self.floors = floors

    def copy(self) -> Self:
        return copy.deepcopy(self)


if __name__ == "__main__":
    lines = lib.read_input_file("11")
    floors = Floors(lines)
    floors_1 = floors.copy()
    floors_1.floors[1].pop()
    print(f"Elevator {floors.elevator}")
    for (floor, components) in floors_1.floors.items():
        print(f"Floor {floor}")
        for component in components:
            print(f"\t{component}")
