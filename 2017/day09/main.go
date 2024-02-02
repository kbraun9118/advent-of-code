package main

import (
	"aoc/2017/lib"
)

type Group struct {
	depth   int
	groups  []Group
	garbage int
}

func (group *Group) Score() int {
	score := group.depth
	for _, group := range group.groups {
		score += group.Score()
	}
	return score
}

func (group *Group) GarbageRemoved() int {
	count := group.garbage
	for _, group := range group.groups {
		count += group.GarbageRemoved()
	}
	return count
}

func removeGarbage(input string, sPointer int) (int, int) {
	count := 0
	for {
		if input[sPointer] == '!' {
			sPointer += 2
		} else if input[sPointer] == '>' {
			return sPointer + 1, count
		} else {
			count++
			sPointer += 1
		}
	}
}

func parseInner(input string, depth, sPointer int) (Group, int) {
	groups := make([]Group, 0)
	garbageCount := 0
	for {
		if sPointer >= len(input)-1 || input[sPointer] == '}' {
			group := Group{
				groups:  groups,
				depth:   depth,
				garbage: garbageCount,
			}
			if sPointer <= len(input)-2 && input[sPointer+1] == ',' {
				return group, sPointer + 2
			} else {
				return group, sPointer + 1
			}
		} else if input[sPointer] == '<' {
			var garbageRemoved int
			sPointer, garbageRemoved = removeGarbage(input, sPointer+1)
			garbageCount += garbageRemoved
		} else if input[sPointer] == '{' {
			var group Group
			group, sPointer = parseInner(input, depth+1, sPointer+1)
			groups = append(groups, group)
		} else {
			sPointer++
		}
	}
}

func parse(input string) Group {
	group, _ := parseInner(input, 0, 0)
	return group
}

func main() {
	input := lib.ReadInputFile("09")[0]
	group := parse(input)

	lib.PrintPart1(group.Score())
	lib.PrintPart1(group.GarbageRemoved())
}
