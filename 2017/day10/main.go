package main

import (
	"aoc/2017/lib"
	"fmt"
	"strconv"
	"strings"
)

type RingBuffer struct {
	buffer []int
	length int
}

func (rb *RingBuffer) realIndex(index int) int {
	return index % rb.length
}

func (rb *RingBuffer) ReverseRange(start, end int) {
	for i := 0; i < (end-start)/2; i++ {
		startI := rb.realIndex(i + start)
		endI := rb.realIndex(end - i - 1)
		rb.buffer[startI], rb.buffer[endI] = rb.buffer[endI], rb.buffer[startI]
	}
}

func CreateRingBuffer(length int) RingBuffer {
	buffer := make([]int, length)
	for i := range buffer {
		buffer[i] = i
	}
	return RingBuffer{
		buffer,
		length,
	}
}

func part1(inputs []int, bufferLength int) int {
	position := 0
	skip := 0
	list := CreateRingBuffer(bufferLength)

	for _, length := range inputs {
		list.ReverseRange(position, position+length)
		position += length + skip
		skip++
	}

	return list.buffer[0] * list.buffer[1]
}

func part2(input string) string {
	position := 0
	skip := 0
	list := CreateRingBuffer(256)
	inputAscii := make([]int, 0)
	for _, ch := range input {
		inputAscii = append(inputAscii, int(ch))
	}
	endSeq := []int{17, 31, 73, 47, 23}
	for _, i := range endSeq {
		inputAscii = append(inputAscii, i)
	}

	for i := 0; i < 64; i++ {
		for _, length := range inputAscii {
			list.ReverseRange(position, position+length)
			position += length + skip
			skip++
		}
	}
	denseHash := make([]int, 16)

	for i := 0; i < 16; i++ {
		denseHash[i] = 0
		for j := 0; j < 16; j++ {
			denseHash[i] ^= list.buffer[i*16+j]
		}
	}

	output := ""

	for _, out := range denseHash {
		output += fmt.Sprintf("%0.2x", out)
	}

	return output
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
