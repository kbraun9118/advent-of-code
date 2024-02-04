package lib

import (
	"fmt"
	"os"
	"strings"
)

func Map[T any, V any](values []T, mapper func(t T) V) []V {
	vs := make([]V, len(values))
	for i, value := range values {
		vs[i] = mapper(value)
	}
	return vs
}

func ReadInputFile(day string) []string {
	content, err := os.ReadFile(fmt.Sprintf("../input/2017/%s.txt", day))
	if err != nil {
		panic("Could not open file")
	}

	lines := string(content)

	return strings.Split(strings.TrimSpace(lines), "\n")
}

func ReadTestFile(day string) []string {
	content, err := os.ReadFile(fmt.Sprintf("./test/%s.txt", day))
	if err != nil {
		panic("Could not open file")
	}

	lines := string(content)

	return strings.Split(strings.TrimSpace(lines), "\n")
}

func PrintPart1(output any) {
	fmt.Printf("Part 1: %+v\n", output)
}

func PrintPart2(output any) {
	fmt.Printf("Part 2: %+v\n", output)
}

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

func (rb RingBuffer) Score() int {
	return rb.buffer[0] * rb.buffer[1]
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

func KnotHash(input string) string {
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
