package main

import (
	"aoc/2017/lib"
	"fmt"
	"strconv"
)

type Coord struct {
	X int
	Y int
}

type Disc [][]bool

func (disc Disc) Neighbors(coord Coord) []Coord {
	neighbors := make([]Coord, 0)
	if coord.X > 0 {
		neighbors = append(neighbors, Coord{X: coord.X - 1, Y: coord.Y})
	}
	if coord.Y > 0 {
		neighbors = append(neighbors, Coord{X: coord.X, Y: coord.Y - 1})
	}
	if coord.X < len(disc[0])-1 {
		neighbors = append(neighbors, Coord{X: coord.X + 1, Y: coord.Y})
	}
	if coord.Y < len(disc)-1 {
		neighbors = append(neighbors, Coord{X: coord.X, Y: coord.Y + 1})
	}

	return neighbors
}

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

func part2(disc Disc) int {
	groups := 0
	visited := make(map[Coord]bool)
	for y, row := range disc {
		for x, v := range row {
			coord := Coord{X: x, Y: y}
			if v && !visited[coord] {
				groups++
				current := []Coord{coord}
				for len(current) > 0 {
					next := make([]Coord, 0)
					for _, node := range current {
						visited[node] = true
						for _, neighbor := range disc.Neighbors(node) {
							if disc[neighbor.Y][neighbor.X] && !visited[neighbor] {
								next = append(next, neighbor)
							}
						}
					}
					current = next
				}
			}
		}
	}
	return groups
}

func main() {
	input := lib.ReadInputFile("14")[0]
	disc := NewDisc(input)

	lib.PrintPart1(part1(disc))
	lib.PrintPart2(part2(disc))
}
