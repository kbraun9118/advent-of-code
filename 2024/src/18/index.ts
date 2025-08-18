import { MinHeap } from "@datastructures-js/heap";
import util, { HashSet, Position } from "../util";

function stepsToEnd(
  bytes: Position[],
  corruptedSteps: number,
  isTest: boolean = false,
): number {
  const queue = new MinHeap<Position & { steps: number }>((v) => v.steps);
  const visited = new HashSet<Position>();
  const target = isTest ? { x: 6, y: 6 } : { x: 70, y: 70 };
  const corrupted = bytes.slice(0, corruptedSteps);

  queue.push({ x: 0, y: 0, steps: 0 });
  visited.add({ x: 0, y: 0 });

  while (queue.size() != 0) {
    const u = queue.pop()!;

    if (u.x === target.x && u.y === target.y) {
      return u.steps;
    }

    const possibleNeighbors = util
      .neighbors(u)
      //within bounds
      .filter((n) => n.x >= 0 && n.x <= target.x)
      .filter((n) => n.y >= 0 && n.y <= target.y)
      // has not been visited
      .filter((n) => !visited.has(n))
      //not one of the corrupted bytes
      .filter((n) => !corrupted.find((v) => v.x === n.x && v.y === n.y));

    for (const neighbor of possibleNeighbors) {
      visited.add({ x: neighbor.x, y: neighbor.y });
      queue.push({ ...neighbor, steps: u.steps + 1 });
    }
  }

  return -1;
}

const isTest = false;

const bytes: Position[] = util
  .readInput("18", isTest)
  .map((l) => l.split(","))
  .map((s) => ({ x: +s[0], y: +s[1] }));

const corruptedSteps = isTest ? 12 : 1024;

const step1 = stepsToEnd(bytes, corruptedSteps, isTest);

let step2 = corruptedSteps;
//ideally this would use binary search, but I'm too lazy
for (; step2 < bytes.length; step2++) {
  if (stepsToEnd(bytes, step2, isTest) === -1) {
    break;
  }
}

util.writeOutput(step1, util.positionString(bytes[step2 - 1]));
