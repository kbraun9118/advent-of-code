package main

import (
	"aoc/2017/lib"
	"math"
	"strconv"
)

func layer_count(layer int) int {
	if layer == 0 {
		return 1
	}

	return layer * 8
}

func layer_values(layer int) (int, int) {
	if layer == 0 {
		return 1, 1
	}

	_, prev_max := layer_values(layer - 1)
	return prev_max + 1, prev_max + layer_count(layer)
}

func part1(num int) int {
	layer := 0
	minN, maxN := layer_values(layer)
	for num > maxN {
		layer++
		minN, maxN = layer_values(layer)
	}

	sideLength := (maxN-minN)/4 + 1

	half := sideLength/2 - 1

	diff := num - minN

	offset := diff % sideLength

	diffLayer := int(math.Abs(float64(offset) - float64(half)))

	return layer + diffLayer
}

type Direction int

const (
	Up Direction = iota
	Down
	Left
	Right
)

type Coord struct {
	X int
	Y int
}

func (c Coord) Move(direction Direction) Coord {
	switch direction {
	case Up:
		return Coord{X: c.X, Y: c.Y - 1}
	case Down:
		return Coord{X: c.X, Y: c.Y + 1}
	case Left:
		return Coord{X: c.X - 1, Y: c.Y}
	case Right:
		return Coord{X: c.X + 1, Y: c.Y}
	}

	panic("Invalid direction")
}

func (d Direction) TurnLeft() Direction {
	switch d {
	case Up:
		return Left
	case Left:
		return Down
	case Down:
		return Right
	default:
		return Up
	}
}

type Grid struct {
	grid             map[Coord]int
	currentCoord     Coord
	currentDirection Direction
}

func CreateGrid() Grid {
	grid := make(map[Coord]int)
	currentCoord := Coord{X: 0, Y: 0}
	grid[currentCoord] = 1

	return Grid{
		grid:             grid,
		currentCoord:     currentCoord,
		currentDirection: Down,
	}
}

func (g *Grid) Move() int {
	nextDirection := g.currentDirection.TurnLeft()
	nextCoord := g.currentCoord.Move(nextDirection)
	if _, ok := g.grid[nextCoord]; ok {
		nextCoord = g.currentCoord.Move(g.currentDirection)
		nextDirection = g.currentDirection
	}
	newValue := g.NeighborValues(nextCoord)
	g.grid[nextCoord] = newValue
	g.currentCoord = nextCoord
	g.currentDirection = nextDirection
	return newValue
}

func (g Grid) NeighborValues(coord Coord) int {
	sum := 0
	for y := -1; y < 2; y++ {
		for x := -1; x < 2; x++ {
			if x == 0 && y == 0 {
				continue
			}
			if value, ok := g.grid[Coord{x + coord.X, y + coord.Y}]; ok {
				sum += value
			}
		}
	}

	return sum
}

func part2(value int) int {
	grid := CreateGrid()

	next := 0
	for next < value {
		next = grid.Move()
	}

	return next
}

func main() {
	line, _ := strconv.Atoi(lib.ReadInputFile("03")[0])

	lib.PrintPart1(part1(line))
	lib.PrintPart1(part2(line))
}
