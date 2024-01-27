package main

import (
	"aoc/2017/lib"
	"strconv"
	"strings"
)

func part1(input string) int {
	chs := strings.Split(input, "")
	output := 0
	for i, ch := range chs {
		if i == len(chs)-1 {
			if ch == chs[0] {
				num, _ := strconv.Atoi(ch)
				output += num
			}
		} else {
      if ch == chs[i+1] {
        num, _ := strconv.Atoi(ch)
        output += num
      }
    }
	}
	return output
}

func part2(input string) int {
  chs := strings.Split(input, "")
  output := 0
  step := len(chs) / 2
  for i, ch := range(chs[:step]) {
    if ch == chs[i+step] {
      num, _ := strconv.Atoi(ch)
      output += num * 2
    }
  }

  return output
}

func main() {
	input := lib.ReadInputFile("01")[0]

	lib.PrintPart1(part1(input))
  lib.PrintPart2(part2(input))
}
