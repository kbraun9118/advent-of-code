package main

import (
	"aoc/2017/lib"
	"reflect"
	"strings"
)

func noDuplicates(s string) bool {
	password := make(map[string]interface{})
	split := strings.Split(s, " ")
	for _, word := range split {
		if _, ok := password[word]; ok {
			return false
		}
		password[word] = nil
	}
	return true
}

func part1(lines []string) int {
	count := 0
	for _, line := range lines {
		if noDuplicates(line) {
			count++
		}
	}

	return count
}

func noAnagram(s string) bool {
	split := strings.Split(s, " ")
	wordMaps := make([]map[rune]int, 0)
	for _, word := range split {
		wordMap := make(map[rune]int)
		for _, r := range word {
			wordMap[r] = wordMap[r] + 1
		}
		wordMaps = append(wordMaps, wordMap)
	}
	for i, wordMap := range wordMaps {
		for _, other := range wordMaps[i+1:] {
			if reflect.DeepEqual(wordMap, other) {
				return false
			}
		}
	}
	return true
}

func part2(lines []string) int {
	count := 0
	for _, line := range lines {
		if noAnagram(line) {
			count++
		}
	}

	return count
}

func main() {
	lines := lib.ReadInputFile("04")

	lib.PrintPart1(part1(lines))
	lib.PrintPart2(part2(lines))
}
