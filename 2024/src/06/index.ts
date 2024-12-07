import util from "../util";
import cliProgress from 'cli-progress';

type Lab = string[][];
type Position = { x: number; y: number };
const turn: Record<string, string> = { "^": ">", ">": "v", v: "<", "<": "^" };

function step(
  lab: Lab,
  currentPos: Position,
  path: Map<string, string[]> = new Map(),
): [Lab, Position | undefined, boolean] {
  const newLab = lab.map((l) => [...l]);

  const current = lab[currentPos.y][currentPos.x];
  const currentPosString = `(${currentPos.x}, ${currentPos.y})`
  const currentPath = path.get(currentPosString);
  if (currentPath) {
    if (currentPath.includes(current)) {
      return [newLab, undefined, true];
    }
    path.set(currentPosString, [...currentPath, current]);
  } else {
    path.set(currentPosString, [current]);
  }
  let nextPos: Position = { x: 0, y: 0 };

  if (current === "^") {
    nextPos = { x: currentPos.x, y: currentPos.y - 1 };
  }
  if (current === ">") {
    nextPos = { x: currentPos.x + 1, y: currentPos.y };
  }
  if (current === "v") {
    nextPos = { x: currentPos.x, y: currentPos.y + 1 };
  }
  if (current === "<") {
    nextPos = { x: currentPos.x - 1, y: currentPos.y };
  }
  if (
    nextPos.x === -1 ||
    nextPos.y === -1 ||
    nextPos.x === lab[0].length ||
    nextPos.y === lab.length
  ) {
    return [newLab, undefined, false];
  }

  const next = lab[nextPos.y][nextPos.x];
  if (next === "#") {
    newLab[currentPos.y][currentPos.x] = turn[current];
    return [newLab, currentPos, false];
  }
  newLab[currentPos.y][currentPos.x] = "X";
  newLab[nextPos.y][nextPos.x] = current;
  return [newLab, nextPos, false];
}

function solvePart1(input: Lab, starting: Position): number {
  let [currentLab, currentPos] = step(input, starting);

  while (currentPos) {
    [currentLab, currentPos] = step(currentLab, currentPos);
  }

  return (
    currentLab.reduce(
      (acc, curr) => acc + curr.filter((c) => c === "X").length,
      0,
    ) + 1
  );
}

function solvePart2(input: Lab, starting: Position): number {
  let count = 0;
  const bar = new cliProgress.SingleBar({}, cliProgress.Presets.shades_classic);
  bar.start(input.length * input[0].length, 0);

  for (let y = 0; y < input.length; y++) {
    for (let x = 0; x < input[y].length; x++) {
      bar.increment();
      if (input[y][x] === ".") {
        const newInput = input.map((i) => [...i]);
        newInput[y][x] = "#";
        const path = new Map<string, string[]>();
        let [currentLab, currentPos, hasLoop] = step(newInput, starting, path);
        while (currentPos) {
          [currentLab, currentPos, hasLoop] = step(
            currentLab,
            currentPos,
            path,
          );
        }
        if (hasLoop) {
          count++;
        }
      }
    }
  }
  bar.stop();
  return count;
}

const input = util.readInput("06").map((s) => s.split(""));
const starting = input
  .map((l, y) => l.map((p, x) => ({ x, y, p })).find((p) => p.p === "^"))
  .find((l) => l)!;

const part1 = solvePart1(input, starting);
const part2 = solvePart2(input, starting);

util.writeOutput(part1, part2);
