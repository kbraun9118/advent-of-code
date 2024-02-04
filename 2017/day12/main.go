package main

import (
	"aoc/2017/lib"
	"strings"
)

type PipeMap map[string][]string

func parse(lines []string) PipeMap {
	pipeMap := make(PipeMap)

	for _, line := range lines {
		split := strings.Split(line, " <-> ")
		pipeMap[split[0]] = strings.Split(split[1], ", ")
	}

	return pipeMap
}

func part1(pipeMap PipeMap) int {
	visited := make(map[string]bool)
	current := []string{"0"}
	visited["0"] = true

	for len(current) > 0 {
		next := make([]string, 0)
		for _, node := range current {
			for _, neightbor := range pipeMap[node] {
				if !visited[neightbor] {
					next = append(next, neightbor)
					visited[neightbor] = true
				}
			}
		}
		current = next
	}

	return len(visited)
}

func part2(pipeMap PipeMap) int {
	visited := make(map[string]bool)
	groupCount := 0
	for key := range pipeMap {
		if !visited[key] {
			groupCount++
			current := []string{key}
			visited[key] = true

			for len(current) > 0 {
				next := make([]string, 0)
				for _, node := range current {
					for _, neightbor := range pipeMap[node] {
						if !visited[neightbor] {
							next = append(next, neightbor)
							visited[neightbor] = true
						}
					}
				}
				current = next
			}
		}
	}

	return groupCount
}

func main() {
	lines := lib.ReadInputFile("12")
	pipeMap := parse(lines)

	lib.PrintPart1(part1(pipeMap))
	lib.PrintPart2(part2(pipeMap))
}
