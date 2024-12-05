package main

import (
	"aoc/2017/lib"
	"slices"
	"strconv"
)

type CircleBuffer struct {
	buffer []int
	length int
}

func NewCircleBuffer(lenght int) CircleBuffer {
	buffer := make([]int, lenght)
	return CircleBuffer{buffer, lenght}
}

func (cb CircleBuffer) RealIndex(index int) int {
	return index % cb.length
}

func (cb CircleBuffer) Get(index int) int {
	return cb.buffer[cb.RealIndex(index)]
}

func (cb *CircleBuffer) InsertAfter(index, value int) {
	cb.buffer = slices.Insert(cb.buffer, cb.RealIndex(index)+1, value)
	cb.length++
}

func part1(offset int) int {
	circleBuffer := NewCircleBuffer(1)
	cbIndex := 0

	for i := 1; i < 2018; i++ {
		cbIndex += offset
		cbIndex = circleBuffer.RealIndex(cbIndex)
		circleBuffer.InsertAfter(cbIndex, i)
		cbIndex++
	}

	return circleBuffer.Get(cbIndex + 1)
}

func part2(offset int) int {
	index1 := -1
	length := 1
	index := 1
	for i := range 50_000_001 {
		index = ((index + offset) % length) + 1
		length++
		if index == 1 {
			index1 = i + 1
		}
	}
	return index1
}

func main() {
	input, _ := strconv.Atoi(lib.ReadInputFile("17")[0])

	lib.PrintPart1(part1(input))
	lib.PrintPart2(part2(input))
}
