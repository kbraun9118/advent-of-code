from enum import Enum
import lib

class Type(Enum):
    Generator = 0
    Microchip = 1

    
class Component:
    def __init__(self, type: Type, value: str) -> None:
        self.type = type
        self.value = value

    def __str__(self) -> str:
        return f"Component(type={self.type}, value={self.value})"
    

if __name__ == "__main__":
    lines = lib.read_input_file("11")
    component = Component(Type.Generator, "hydrogen")
    print(component)
