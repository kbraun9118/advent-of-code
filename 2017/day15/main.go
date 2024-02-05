package main

import (
	"aoc/2017/lib"
	"fmt"
	"math/big"
	"strconv"
	"strings"
)

type Generator struct {
	factor big.Int
	value  big.Int
}

func NewGenerator(value, factor int) Generator {
	return Generator{
		value:  *big.NewInt(int64(value)),
		factor: *big.NewInt(int64(factor)),
	}
}

func (generator *Generator) Next() big.Int {
	generator.value.Mul(&generator.value, &generator.factor).Rem(&generator.value, big.NewInt(2147483647))
	return generator.value
}

func (generator *Generator) NextDiv(div int) big.Int {
	generator.Next()
	divBig := big.NewInt(int64(div))
	rem := big.NewInt(0).Rem(&generator.value, divBig)
	for {
		if rem.Cmp(big.NewInt(0)) == 0 {
			return generator.value
		}
		generator.Next()
		rem = big.NewInt(0).Rem(&generator.value, divBig)
	}
}

func (generator *Generator) Last16() string {
	num := fmt.Sprintf("%016s", generator.value.Text(2))

	return num[len(num)-16:]
}

func parse(line string, factor int) Generator {
	split := strings.Split(line, " ")[4]
	num, _ := strconv.Atoi(split)
	return NewGenerator(num, factor)
}

func part1(a, b Generator) int {
	count := 0
	for i := 0; i < 40_000_000; i++ {
		a.Next()
		b.Next()

		if a.Last16() == b.Last16() {
			count++
		}
	}
	return count
}

func part2(a, b Generator) int {
	count := 0
	for i := 0; i < 5_000_000; i++ {
		a.NextDiv(4)
		b.NextDiv(8)

		if a.Last16() == b.Last16() {
			count++
		}
	}
	return count
}

func main() {
	lines := lib.ReadInputFile("15")
	a := parse(lines[0], 16807)
	b := parse(lines[1], 48271)

	lib.PrintPart1(part1(a, b))
	a = parse(lines[0], 16807)
	b = parse(lines[1], 48271)

	lib.PrintPart2(part2(a, b))
}
