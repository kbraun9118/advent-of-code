package main

import (
	"aoc/2017/lib"
	"fmt"
	"slices"
	"strconv"
	"strings"
)

type Block struct {
	banks string
}

func BlocksCreate(ints []int) Block {
	banks := fmt.Sprintf("%v", ints)
	return Block{banks}
}

func Redistribute(ints []int) {
	max := slices.Max(ints)
	i := slices.Index(ints, max)
	ints[i] = 0

	for max > 0 {
		i = (i + 1) % len(ints)
		ints[i]++
		max--
	}
}

func runBlocks(banks []int) (int, int) {
	set := make(map[Block]bool)
	blocks := make([]Block, 0)
	i := 0
	for {
		Redistribute(banks)

		block := BlocksCreate(banks)
		i++
		if set[block] {
			blockI := slices.Index(blocks, block)
			return i, len(blocks) - blockI

		}
		set[block] = true
		blocks = append(blocks, block)
	}
}

func main() {
	input := lib.ReadInputFile("06")[0]
	split := strings.Split(input, "\t")
	banks := lib.Map(split, func(s string) int {
		num, _ := strconv.Atoi(s)
		return num
	})
	part1, part2 := runBlocks(banks)
	lib.PrintPart1(part1)
	lib.PrintPart2(part2)
}
