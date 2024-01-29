package main

import (
	"aoc/2017/lib"
	"math"
	"strconv"
	"strings"
)

type Program struct {
	id       string
	weight   int
	parent   *Program
	children []*Program
}

func NewProgram(id string, weight int) Program {
	return Program{
		id:       id,
		weight:   weight,
		parent:   nil,
		children: make([]*Program, 0),
	}
}

func (p *Program) Add(child *Program) {
	child.parent = p
	p.children = append(p.children, child)
}

func (p Program) Weight() int {
	childWeight := 0
	for _, child := range p.children {
		childWeight += child.Weight()
	}

	return p.weight + childWeight
}

func parse(inputs []string) Program {
	splits := lib.Map(inputs, func(input string) []string {
		return strings.Split(input, " -> ")
	})
	programs := make(map[string]*Program)
	for _, split := range splits {
		idSplits := strings.Split(split[0], " (")
		weight, _ := strconv.Atoi(idSplits[1][:len(idSplits[1])-1])
		program := NewProgram(idSplits[0], weight)
		programs[program.id] = &program
	}
	for _, split := range splits {
		id := strings.Split(split[0], " ")[0]
		parent := programs[id]
		if len(split) > 1 {
			for _, childId := range strings.Split(split[1], ", ") {
				child := programs[childId]
				parent.Add(child)
			}
		}
	}
	var current *Program
	for _, program := range programs {
		current = program
	}
	for current.parent != nil {
		current = current.parent
	}

	return *current
}

func part2(program Program) int {
	weights := lib.Map(program.children, func(p *Program) int {
		weight := p.Weight()
		println(weight)
		return weight
	})
	for i, weight := range weights {
		if weight != weights[i+1] {
			return int(math.Abs(float64(weight - weights[i+1])))
		}
	}

	return -1
}

func main() {
	lines := lib.ReadInputFile("07")
	program := parse(lines)

	lib.PrintPart1(program.id)
	lib.PrintPart2(part2(program))
}
