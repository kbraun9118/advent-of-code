package main

import (
	"aoc/2017/lib"
	"fmt"
	"math"
	"strconv"
	"strings"
)

type Direction int

const (
	Up   Direction = 1
	Down           = -1
)

type Firewall struct {
	layers         []int
	layerDirection []Direction
	layerPosition  []int
	packet         int
}

func NewFirewall(lines []string) Firewall {
	layerMap := make(map[int]int)
	for _, line := range lines {
		split := strings.Split(line, ": ")
		key, _ := strconv.Atoi(split[0])
		value, _ := strconv.Atoi(split[1])
		layerMap[key] = value
	}
	maxKey := 0
	for key := range layerMap {
		maxKey = int(math.Max(float64(maxKey), float64(key)))
	}
	layers := make([]int, maxKey+1)
	layerDirection := make([]Direction, maxKey+1)
	layerPosition := make([]int, maxKey+1)
	for i := 0; i <= maxKey; i++ {
		layers[i] = layerMap[i]
		layerDirection[i] = Up
	}
	return Firewall{
		layers:         layers,
		layerDirection: layerDirection,
		layerPosition:  layerPosition,
		packet:         0,
	}
}

func (firewall *Firewall) Tick() {
	for i := range firewall.layers {
		firewall.layerPosition[i] += int(firewall.layerDirection[i])
		if firewall.layerPosition[i] == 0 {
			firewall.layerDirection[i] = Up
		}
		if firewall.layerPosition[i] == firewall.layers[i]-1 {
			firewall.layerDirection[i] = Down
		}
	}
}

func main() {
	lines := lib.ReadTestFile("13")
	layerMap := NewFirewall(lines)

	// lib.PrintPart1(layerMap)
	fmt.Println(layerMap.layerPosition)
	for i := 0; i < 3; i++ {
		layerMap.Tick()
		fmt.Println(layerMap.layerPosition)
	}
}
