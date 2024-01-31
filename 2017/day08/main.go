package main

import "aoc/2017/lib"

type Instruction int

const (
	GreaterThan Instruction = iota
	GreaterThanEqual
	LessThan
	LessthanEqual
	Equal
	NotEqual
)

func main() {
	lines := lib.ReadTestFile("08")
	println("Hello")
}
