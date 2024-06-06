package main

import (
	"aoc/2017/lib"
	"slices"
	"strconv"
	"strings"
)

type Program struct {
	programs []string
}

func NewProgram(program []string) Program {
	return Program{program}
}

func DefaultProgram() Program {
	programs := make([]string, 0)

	for i := 0; i < 16; i++ {
		programs = append(programs, string(rune(97+i)))
	}

	return Program{programs}
}

type dance interface {
	Move(program *Program)
}

type Spin struct {
	distance int
}

func (spin Spin) Move(program *Program) {
	current := program.programs
	for i := 0; i < spin.distance; i++ {
		current = slices.Insert(current, 0, current[len(current)-1])
		current = current[:len(current)-1]
	}
	program.programs = current
}

type Exchange struct {
	src  int
	dest int
}

func (exchange Exchange) Move(program *Program) {
	program.programs[exchange.src], program.programs[exchange.dest] = program.programs[exchange.dest], program.programs[exchange.src]
}

type Partner struct {
	src  string
	dest string
}

func (partner Partner) Move(program *Program) {
	exchange := Exchange{
		src:  slices.Index(program.programs, partner.src),
		dest: slices.Index(program.programs, partner.dest),
	}
	exchange.Move(program)
}

func parse(input string) []dance {
	dances := make([]dance, 0)
	inputs := strings.Split(input, ",")
	for _, i := range inputs {
		switch i[0] {
		case 's':
			distance, _ := strconv.Atoi(i[1:])
			dances = append(dances, Spin{distance})
		case 'x':
			split := strings.Split(i[1:], "/")
			src, _ := strconv.Atoi(split[0])
			dest, _ := strconv.Atoi(split[1])
			dances = append(dances, Exchange{src, dest})
		case 'p':
			split := strings.Split(i[1:], "/")
			dances = append(dances, Partner{src: split[0], dest: split[1]})

		}
	}

	return dances
}

func part1(dances []dance) string {
	program := DefaultProgram()
	for _, dance := range dances {
		dance.Move(&program)
	}
	output := ""
	for _, out := range program.programs {
		output += out
	}
	return output
}

func part2(dances []dance) string {
	program := DefaultProgram()
	output := ""
	outputMap := make(map[string]int)

	for i := 0; true; i++ {
		output = ""
		for _, dance := range dances {
			dance.Move(&program)
		}
		for _, out := range program.programs {
			output += out
		}
		if _, ok := outputMap[output]; ok {
			break
		}
		outputMap[output] = i
	}
	loopLen := len(outputMap)
	rem := 1_000_000_000 % loopLen

	for key, v := range outputMap {
		if v == rem-1 {
			return key
		}
	}

	return ""
}

func main() {
	input := lib.ReadInputFile("16")[0]

	dances := parse(input)

	lib.PrintPart1(part1(dances))
	lib.PrintPart2(part2(dances))

}
