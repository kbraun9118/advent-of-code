import util from "../util";

type Lab = string[][];
type Position = { x: number; y: number };

function step(lab: Lab, currentPos: Position): [Lab, Position | undefined] {
  const newLab = lab.map((l) => [...l]);
  const current = lab[currentPos.y][currentPos.x];
  console.log(currentPos);

  if (!current) {
    return [newLab, undefined];
  }

  if (current === "^") {
    const nextPos = { x: currentPos.x, y: currentPos.y - 1 };
    const next = lab[nextPos.y][nextPos.x];
    if (next === "#") {
      newLab[currentPos.y][currentPos.x] = ">";
      return [newLab, currentPos];
    }
    newLab[currentPos.y][currentPos.x] = "#";
    newLab[nextPos.y][nextPos.x] = "^";
    return [newLab, nextPos];
  }
  if (current === ">") {
    const nextPos = { x: currentPos.x + 1, y: currentPos.y };
    const next = lab[nextPos.y][nextPos.x];
    if (next === "#") {
      newLab[currentPos.y][currentPos.x] = "v";
      return [newLab, currentPos];
    }
    newLab[currentPos.y][currentPos.x] = "#";
    newLab[nextPos.y][nextPos.x] = ">";
    return [newLab, nextPos];
  }
  if (current === "v") {
    const nextPos = { x: currentPos.x, y: currentPos.y + 1 };
    const next = lab[nextPos.y][nextPos.x];
    if (next === "#") {
      newLab[currentPos.y][currentPos.x] = "<";
      return [newLab, currentPos];
    }
    newLab[currentPos.y][currentPos.x] = "#";
    newLab[nextPos.y][nextPos.x] = "v";
    return [newLab, nextPos];
  }
  if (current === "<") {
    const nextPos = { x: currentPos.x - 1, y: currentPos.y };
    const next = lab[nextPos.y][nextPos.x];
    if (next === "#") {
      newLab[currentPos.y][currentPos.x] = "^";
      return [newLab, currentPos];
    }
    newLab[currentPos.y][currentPos.x] = "#";
    newLab[nextPos.y][nextPos.x] = "<";
    return [newLab, nextPos];
  }

  return [newLab, undefined];
}

function solvePart1(input: Lab, starting: Position): number {
  let [currentLab, currentPos] = step(input, starting);

  while (currentPos) {
    [currentLab, currentPos] = step(currentLab, currentPos);
  }

  return currentLab.reduce(
    (acc, curr) => acc + curr.filter((c) => c === "X").length,
    0,
  );
}

const input = util.readInput("06", true).map((s) => s.split(""));
const starting = input
  .map((l, y) => l.map((p, x) => ({ x, y, p })).find((p) => p.p === "^"))
  .find((l) => l)!;

const part1 = solvePart1(input, starting);

util.writeOutput(part1);
