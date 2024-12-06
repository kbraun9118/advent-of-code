package main

import (
	"aoc/2017/lib"
	"strconv"
	"strings"
)

type Command int

const (
	Send Command = iota
	Set
	Add
	Mul
	Mod
	Recover
	JumpGreater
)

type Instruction struct {
	Command Command
	Left    string
	Right   string
}

type Duet struct {
	Registers          map[string]int
	Instructions       []Instruction
	InstructionPointer int
	PreviousFrequency  int
}

func NewDuet(instructionStrings []string) Duet {
	registers := make(map[string]int)
	instructions := make([]Instruction, 0)

	for _, instructionString := range instructionStrings {
		split := strings.Split(instructionString, " ")
		var command Command
		switch split[0] {
		case "snd":
			command = Send
		case "set":
			command = Set
		case "add":
			command = Add
		case "mul":
			command = Mul
		case "mod":
			command = Mod
		case "rcv":
			command = Recover
		case "jgz":
			command = JumpGreater
		}
		left := split[1]
		if _, err := strconv.Atoi(left); err != nil {
			registers[left] = 0
		}
		var right string
		if len(split) == 3 {
			right = split[2]
			if _, err := strconv.Atoi(right); err != nil {
				registers[right] = 0
			}
		}
		instructions = append(instructions, Instruction{
			Command: command,
			Right:   right,
			Left:    left,
		})
	}

	return Duet{
		Instructions:       instructions,
		Registers:          registers,
		InstructionPointer: 0,
		PreviousFrequency:  -1,
	}
}

func runUntilRecover(duet Duet) int {
	for {
		instruction := duet.Instructions[duet.InstructionPointer]
		var left, right int
		if l, err := strconv.Atoi(instruction.Left); err == nil {
			left = l
		} else {
			left = duet.Registers[instruction.Left]
		}
		if instruction.Right != "" {
			if r, err := strconv.Atoi(instruction.Right); err == nil {
				right = r
			} else {
				right = duet.Registers[instruction.Right]
			}
		}
		switch instruction.Command {
		case Send:
			duet.PreviousFrequency = left
			duet.InstructionPointer++
		case Set:
			duet.Registers[instruction.Left] = right
			duet.InstructionPointer++
		case Add:
			duet.Registers[instruction.Left] += right
			duet.InstructionPointer++
		case Mul:
			duet.Registers[instruction.Left] *= right
			duet.InstructionPointer++
		case Mod:
			duet.Registers[instruction.Left] %= right
			duet.InstructionPointer++
		case Recover:
			if duet.PreviousFrequency != -1 {
				return duet.PreviousFrequency
			}
			duet.InstructionPointer++
		case JumpGreater:
			if left > 0 {
				duet.InstructionPointer += right
			} else {
				duet.InstructionPointer++
			}
		}
	}
}

func main() {
	input := lib.ReadInputFile("18")
	duet := NewDuet(input)
	lib.PrintPart1(runUntilRecover(duet))
}
