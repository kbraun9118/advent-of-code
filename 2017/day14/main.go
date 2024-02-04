package main

import (
	"aoc/2017/lib"
	"fmt"
	"strconv"
)

type Disc [][]bool

func getHex(input rune) string {
	hex, _ := strconv.ParseInt(string(input), 16, 0)
	return fmt.Sprintf("%0.4b", hex)
}

func NewDisc(input string) Disc {
	disc := make(Disc, 128)

	for i := 0; i < 128; i++ {
		lineHash := lib.KnotHash(fmt.Sprintf("%s-%d", input, i))
		lineBits := make([]bool, 128)
		for j, r := range lineHash {
			hex := getHex(r)
			for k, c := range hex {
				lineBits[j*4+k] = c == '1'
			}
		}
		disc[i] = lineBits
	}
	return disc
}

func part1(disc Disc) int {
	count := 0
	for _, row := range disc {
		for _, set := range row {
			if set {
				count++
			}
		}
	}
	return count
}

func main() {
	input := lib.ReadInputFile("14")[0]
	disc := NewDisc(input)

	lib.PrintPart1(part1(disc))

}
