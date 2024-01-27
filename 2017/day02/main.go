package main

import (
	"aoc/2017/lib"
	"slices"
	"strconv"
	"strings"
)

func parse(lines []string) [][]int {
	output := make([][]int, 0)
	for _, line := range lines {
		split := strings.Split(line, "\t")
		output_line := make([]int, 0)
		for _, num := range split {
			num, _ := strconv.Atoi(num)
			output_line = append(output_line, num)
		}

		output = append(output, output_line)
	}

	return output
}

func part1(inputs [][]int) int {
	output := 0
	for _, input := range inputs {
		output += slices.Max(input) - slices.Min(input)
	}

	return output
}

func part2(inputs [][]int) int {
  output := 0
  for _, input :=range inputs {
    inner:
    for i, num := range input {
      for _, other := range input[i+1:] {
        if other % num == 0 {
          output += other / num
          break inner
        }
        if num % other == 0 {
          output += num / other
          break inner
        }
      }
    }
  }

  return output
}

func main() {
	lines := lib.ReadInputFile("02")

	nums := parse(lines)

	lib.PrintPart1(part1(nums))
	lib.PrintPart2(part2(nums))
}
