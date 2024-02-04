package main

import (
	"aoc/2017/lib"
	"math"
	"strings"
)

type Direction int

const (
	South Direction = iota
	North
	SouthEast
	SouthWest
	NorthEast
	NorthWest
)

func (direction Direction) Move() (float64, float64) {
	switch direction {
	case North:
		return -1, 0
	case South:
		return 1, 0
	case SouthEast:
		return 0.5, 0.5
	case SouthWest:
		return 0.5, -0.5
	case NorthEast:
		return -0.5, 0.5
	case NorthWest:
		return -0.5, -0.5
	default:
		panic("Invalid direction")
	}
}

func ParseDirection(input string) Direction {
	switch input {
	case "n":
		return North
	case "s":
		return South
	case "se":
		return SouthEast
	case "sw":
		return SouthWest
	case "ne":
		return NorthEast
	case "nw":
		return NorthWest
	default:
		panic("Invalid Direction")
	}
}

func stepsAway(directions []Direction) (float64, float64) {
	x, y := 0.0, 0.0
	maxDistance := 0.0
	for _, direction := range directions {
		nX, nY := direction.Move()
		x += nX
		y += nY
		distance := math.Abs(x) + math.Abs(y)
		maxDistance = max(distance, maxDistance)
	}

	return math.Abs(x) + math.Abs(y), maxDistance
}

func main() {
	input := lib.ReadInputFile("11")[0]
	directions := lib.Map(strings.Split(input, ","), ParseDirection)
	part1, part2 := stepsAway(directions)
	lib.PrintPart1(part1)
	lib.PrintPart2(part2)
}
