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

const positionDiffs = [
  { x: 0, y: -1 },
  { x: 0, y: 1 },
  { x: -1, y: 0 },
  { x: 1, y: 0 },
];

function neighbors(position: Position): Position[] {
  return positionDiffs.map((p) => ({
    x: position.x + p.x,
    y: position.y + p.y,
  }));
}

function batchWhile<T>(input: T[], condition: (item: T) => boolean): T[][] {
  const output = [];
  let current = [];
  for (const item of input) {
    if (condition(item)) {
      current.push(item);
    } else {
      output.push(current);
      current = [];
    }
  }
  if (current.length !== 0) {
    output.push(current);
  }
  return output;
}

const util = { readInput, writeOutput, positionString, batchWhile, neighbors };

export default util;
