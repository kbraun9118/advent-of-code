package main

import (
	"aoc/2017/lib"
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

func (p Program) TotalWeight() int {
	childWeight := 0
	for _, child := range p.children {
		childWeight += child.TotalWeight()
	}

	return p.weight + childWeight
}

func (p Program) Unbalanced() *Program {
	for i, child := range p.children[:len(p.children)-2] {
		first := child.TotalWeight()
		second := p.children[i+1].TotalWeight()
		third := p.children[i+2].TotalWeight()
		if first != second && first != third {
			return child
		}
		if second != third && second != first {
			return p.children[i+1]
		}
		if third != second && third != first {
			return p.children[i+2]
		}
	}
	return nil
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
	current := program.Unbalanced()
	next := current.Unbalanced()
	for next != nil {
		current = next
		next = next.Unbalanced()
	}
	next = current
	current = current.parent

	if next == current.children[0] {
		return next.weight + current.children[1].TotalWeight() - next.TotalWeight()
	} else {
		return next.weight + current.children[0].TotalWeight() - next.TotalWeight()
	}
}

func main() {
	lines := lib.ReadInputFile("07")
	program := parse(lines)

	lib.PrintPart1(program.id)
	lib.PrintPart2(part2(program))
}
