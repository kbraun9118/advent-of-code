import { readFileSync } from "fs";

function readInput(day: string, isTest: boolean = false): string[] {
  const file = isTest
    ? `./src/${day}/test.txt`
    : `../input/2024/${day}/input.txt`;
  const lines = readFileSync(file)
    .toString()
    .split(/\r\n|\n/);

  while (lines[lines.length - 1] === "") {
    lines.pop();
  }

  return lines;
}

type Output = string | number;

function writeOutput(part1: Output, part2?: Output) {
  console.log(`Part 1: ${part1}`);

  if (part2) {
    console.log(`Part 2: ${part2}`);
  }
}

export type Position = { x: number; y: number };

function positionString(position: Position) {
  return `(${position.x}, ${position.y})`;
}

const util = { readInput, writeOutput, positionString };

export default util;
