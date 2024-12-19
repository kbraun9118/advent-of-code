import { readFileSync } from "fs";
import stableStringify from "json-stable-stringify";

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

function parsePositionString(position: string): Position {
  const matches = position.match(/\((\d+), (\d+)\)/)!;

  return { x: +matches[1], y: +matches[2] };
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

export class HashMap<K extends object, V> {
  private map: Map<string, V> = new Map();

  constructor() {}

  has(key: K): boolean {
    return this.map.has(stableStringify(key)!);
  }

  get(key: K): V | undefined {
    return this.map.get(stableStringify(key)!);
  }

  set(key: K, value: V): void {
    this.map.set(stableStringify(key)!, value);
  }

  get size(): number {
    return this.map.size;
  }
}

export class HashSet<K extends object> {
  private set: Set<string> = new Set();

  constructor() {}

  get size(): number {
    return this.set.size;
  }
  has(key: K): boolean {
    return this.set.has(stableStringify(key)!);
  }

  add(key: K): void {
    this.set.add(stableStringify(key)!);
  }

  forEach(func: (key: K) => void): void {
    this.set.forEach(k => func(JSON.parse(k)))
  }
}

const util = {
  readInput,
  writeOutput,
  positionString,
  batchWhile,
  neighbors,
  parsePositionString,
};

export default util;
