package main

import (
	"aoc/2017/lib"
	"strconv"
	"strings"
)

func parse(lines []string) map[int]int {
	layerMap := make(map[int]int)
	for _, line := range lines {
		split := strings.Split(line, ": ")
		key, _ := strconv.Atoi(split[0])
		value, _ := strconv.Atoi(split[1])
		layerMap[key] = value
	}
	return layerMap
}

func part1(firewall map[int]int) int {
	severity := 0
	for key, value := range firewall {
		if key%((value-1)*2) == 0 {
			severity += key * value
		}
	}
	return severity
}

func part2(firewall map[int]int) int {
	wait := 0
	for {
		passed := true
	inner:
		for key, value := range firewall {
			if (key+wait)%((value-1)*2) == 0 {
				passed = false
				break inner
			}
		}
		if passed {
			return wait
		}
		wait++
	}

}

func main() {
	lines := lib.ReadInputFile("13")
	layerMap := parse(lines)

	lib.PrintPart1(part1(layerMap))
	lib.PrintPart2(part2(layerMap))
}
