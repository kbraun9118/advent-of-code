package main

import (
	"aoc/2017/lib"
	"slices"
	"strconv"
)

func part1(instructions []int) int {
  i, count := 0, 0
  for i < len(instructions) {
    current := i + instructions[i]
    instructions[i]++
    count++
    i = current
  }

  return count
}

func part2(instructions []int) int {
  i, count := 0, 0
  for i < len(instructions) {
    
    current := i + instructions[i]
    if instructions[i] > 2 {
      instructions[i]--
    } else {
      instructions[i]++
    }
    count++
    i = current
  }

  return count

}

func main() {
	lines := lib.ReadInputFile("05")
  instructions := lib.Map(lines, func(line string) int {
    instruction, _ := strconv.Atoi(line)
    return instruction
  })

	lib.PrintPart1(part1(slices.Clone(instructions)))
  lib.PrintPart2(part2(instructions))
}
