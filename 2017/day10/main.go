package main

import (
	"aoc/2017/lib"
	"strconv"
	"strings"
)

func part1(inputs []int, bufferLength int) int {
	position := 0
	skip := 0
	list := lib.CreateRingBuffer(bufferLength)

	for _, length := range inputs {
		list.ReverseRange(position, position+length)
		position += length + skip
		skip++
	}

	return list.Score()
}

func part2(input string) string {
	return lib.KnotHash(input)
}

func main() {
	input := lib.ReadInputFile("10")[0]
	lengths := lib.Map(strings.Split(input, ","), func(i string) int {
		n, _ := strconv.Atoi(i)
		return n
	})

	lib.PrintPart1(part1(lengths, 256))
	lib.PrintPart2(part2(input))
}
