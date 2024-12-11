import util, { Position } from "../util";

const neighbors: Position[] = [
  { x: 1, y: 0 },
  { x: -1, y: 0 },
  { x: 0, y: 1 },
  { x: 0, y: -1 },
];

function findTrailScore(
  topographicMap: number[][],
  startingPosition: Position,
): number {
  const currentPositions = [startingPosition];
  const hasVisitedSet = new Set<string>();
  const positionSet = new Set<string>();

  while (currentPositions.length !== 0) {
    const current = currentPositions.shift()!;
    const currentHeight = topographicMap[current.y][current.x];
    const currentString = util.positionString(current);

    if (hasVisitedSet.has(currentString)) {
      continue;
    }

    if (currentHeight === 9) {
      positionSet.add(currentString);
      continue;
    }

    for (const next of neighbors.map((p) => ({
      x: current.x + p.x,
      y: current.y + p.y,
    }))) {
      if (
        topographicMap[next.y] &&
        topographicMap[next.y][next.x] === currentHeight + 1
      ) {
        currentPositions.push(next);
      }
    }
  }
  return positionSet.size;
}

function findUniqueTrailScore(
  topographicMap: number[][],
  startingPosition: Position,
): number {
  const currentPositions = [startingPosition];
  const hasVisitedSet = new Set<string>();
  let score = 0;

  while (currentPositions.length !== 0) {
    const current = currentPositions.shift()!;
    const currentHeight = topographicMap[current.y][current.x];
    const currentString = util.positionString(current);

    if (hasVisitedSet.has(currentString)) {
      continue;
    }

    if (currentHeight === 9) {
      score++;
      continue;
    }

    for (const next of neighbors.map((p) => ({
      x: current.x + p.x,
      y: current.y + p.y,
    }))) {
      if (
        topographicMap[next.y] &&
        topographicMap[next.y][next.x] === currentHeight + 1
      ) {
        currentPositions.push(next);
      }
    }
  }
  return score++;
}

const input = util.readInput("10");
const topographicMap = input.map((l) => l.split("").map((h) => +h));

const startingPositions: Position[] = topographicMap.flatMap((l, y) =>
  l
    .map((h, x) => [h, x])
    .filter(([h]) => h === 0)
    .map(([, x]) => ({ x, y })),
);

const part1 = startingPositions.reduce(
  (acc, next) => acc + findTrailScore(topographicMap, next),
  0,
);
const part2 = startingPositions.reduce(
  (acc, next) => acc + findUniqueTrailScore(topographicMap, next),
  0,
);

util.writeOutput(part1, part2);
