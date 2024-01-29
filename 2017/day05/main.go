package main

import (
	"aoc/2017/lib"
	"strconv"
)

func main() {
	lines := lib.ReadInputFile("05")
  instructions := lib.Map(lines, func(line string) int {
    instruction, _ := strconv.Atoi(line)
    return instruction
  })

	lib.PrintPart1(instructions)
}
