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
