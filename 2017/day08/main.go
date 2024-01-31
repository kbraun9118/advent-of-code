package main

import (
	"aoc/2017/lib"
	"math"
	"slices"
	"strconv"
	"strings"

	"golang.org/x/exp/maps"
)

type Conditional int

const (
	GreaterThan Conditional = iota
	GreaterThanEqual
	LessThan
	LessthanEqual
	Equal
	NotEqual
)

func NewConditional(input string) Conditional {
	switch input {
	case ">":
		return GreaterThan
	case ">=":
		return GreaterThanEqual
	case "<":
		return LessThan
	case "<=":
		return LessthanEqual
	case "==":
		return Equal
	case "!=":
		return NotEqual
	default:
		panic("Invalid character")
	}
}

func (c Conditional) test(registerValue, conditionalValue int) bool {
	switch c {
	case GreaterThan:
		return registerValue > conditionalValue
	case GreaterThanEqual:
		return registerValue >= conditionalValue
	case LessThan:
		return registerValue < conditionalValue
	case LessthanEqual:
		return registerValue <= conditionalValue
	case Equal:
		return registerValue == conditionalValue
	case NotEqual:
		return registerValue != conditionalValue
	default:
		return false
	}
}

type Operation int

const (
	Increment Operation = iota
	Decrement
)

type Instruction struct {
	modifyRegister    string
	operation         Operation
	modificationValue int
	conditionRegister string
	conditional       Conditional
	conditionalValue  int
}

func NewInstruction(line string) Instruction {
	split := strings.Split(line, " ")
	modifyRegister := split[0]
	var operation Operation
	if split[1] == "inc" {
		operation = Increment
	} else {
		operation = Decrement
	}
	modificationValue, _ := strconv.Atoi(split[2])
	conditionalRegister := split[4]
	conditional := NewConditional(split[5])
	conditionalValue, _ := strconv.Atoi(split[6])
	return Instruction{
		modifyRegister,
		operation,
		modificationValue,
		conditionalRegister,
		conditional,
		conditionalValue,
	}
}

func RunInstructions(instructions []Instruction) (int, int) {
	registers := make(map[string]int)
	maxEver := 0.0
	for _, instruction := range instructions {
		if instruction.conditional.test(registers[instruction.conditionRegister], instruction.conditionalValue) {
			if instruction.operation == Increment {
				registers[instruction.modifyRegister] += instruction.modificationValue
			} else {
				registers[instruction.modifyRegister] -= instruction.modificationValue
			}
		}
		values := maps.Values(registers)
		if len(values) > 0 {
			maxEver = math.Max(float64(slices.Max(values)), maxEver)
		}
	}

	return slices.Max(maps.Values(registers)), int(maxEver)
}

func main() {
	lines := lib.ReadInputFile("08")
	instructions := lib.Map(lines, func(line string) Instruction { return NewInstruction(line) })

	part1, part2 := RunInstructions(instructions)

	lib.PrintPart1(part1)
	lib.PrintPart2(part2)
}
