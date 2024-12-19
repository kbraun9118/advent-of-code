import { MinHeap } from "@datastructures-js/heap";
import util, { Position } from "../util";

type Direction = "N" | "E" | "W" | "S";

type Reindeer = {
  position: Position;
  facing: Direction;
  cost: number;
};

function reindeerString(reindeer: Reindeer): string {
  return `(${util.positionString(reindeer.position)}, ${reindeer.facing})`;
}

function parseReindeerString(reindeer: string): Reindeer {
  const matches = reindeer.match(/\(\((\d+), (\d+)\), (\w)\)/)!;

  return {
    position: { x: +matches[1], y: +matches[2] },
    facing: matches[3] as Direction,
    cost: 0,
  };
}

class Maze {
  private constructor(
    public map: Set<string>,
    public start: Position,
    public end: Position,
    public height: number,
    public width: number,
  ) {}

  static fromInput(input: string[]): Maze {
    const map = new Set<string>();

    let start: Position = { x: 0, y: 0 };
    let end: Position = { x: 0, y: 0 };
    for (const [y, row] of input.entries()) {
      for (const [x, v] of row.split("").entries()) {
        if (v === "." || v === "E" || v === "S") {
          map.add(util.positionString({ x, y }));
          if (v === "S") {
            start = { x, y };
          }
          if (v === "E") {
            end = { x, y };
          }
        }
      }
    }

    return new Maze(map, start, end, input.length, input[0].length);
  }

  neighbors(position: Position): Position[] {
    return util
      .neighbors(position)
      .filter((p) => this.map.has(util.positionString(p)));
  }

  move(source: Reindeer, destination: Position): Reindeer {
    const positionDiff = {
      x: source.position.x - destination.x,
      y: source.position.y - destination.y,
    };
    if (source.facing === "N" || source.facing === "S") {
      if (positionDiff.x === 0) {
        return {
          position: destination,
          cost: source.cost + 1,
          facing: source.facing,
        };
      }

      return {
        position: destination,
        cost: source.cost + 1001,
        facing: positionDiff.x < 0 ? "E" : "W",
      };
    }

    if (positionDiff.y === 0) {
      return {
        position: destination,
        cost: source.cost + 1,
        facing: source.facing,
      };
    }
    return {
      position: destination,
      cost: source.cost + 1001,
      facing: positionDiff.y < 0 ? "S" : "N",
    };
  }

  lowestCost(): [number, number] {
    const prev = new Map<string, Reindeer[]>();
    const dist = new Map<string, number>();
    const queue = new MinHeap<Reindeer>((r) => r.cost);
    const startReindeer: Reindeer = {
      position: this.start,
      cost: 0,
      facing: "E",
    };
    queue.push(startReindeer);
    prev.set(reindeerString(startReindeer), []);

    while (queue.size() > 0) {
      const current = queue.pop()!;
      if (
        current.position.x === this.end.x &&
        current.position.y === this.end.y
      ) {
        continue;
      }

      for (const neighbor of this.neighbors(current.position)) {
        const next = this.move(current, neighbor);
        const nextReindeer = reindeerString(next);
        if (dist.has(nextReindeer)) {
          const nextDist = dist.get(nextReindeer)!;
          if (nextDist === next.cost) {
            prev.get(nextReindeer)?.push(current);
          }
          continue;
        } else {
          prev.set(nextReindeer, [current]);
          dist.set(nextReindeer, next.cost);
        }

        queue.push(next);
      }
    }

    const paths = new Set<string>();
    const endReindeer: Reindeer[] = (["N", "E", "W", "S"] as Direction[]).map(
      (d) => ({ position: this.end, facing: d, cost: 0 }),
    );
    const minDistance = endReindeer
      .map((r) => dist.get(reindeerString(r)))
      .filter((r) => r !== undefined)
      .reduce((acc, next) => Math.min(acc, next));
    const pathQueue = endReindeer.filter(
      (r) => dist.get(reindeerString(r)) === minDistance,
    );

    while (pathQueue.length > 0) {
      const current = pathQueue.shift()!;
      const prevPaths = prev.get(reindeerString(current))!;
      if (
        current.position.x === this.start.x &&
        current.position.y === this.start.y
      ) {
        continue;
      }
      for (const path of prevPaths) {
        const pathString = reindeerString(path);
        if (paths.has(pathString)) {
          continue;
        }
        paths.add(pathString);
        pathQueue.push(path);
      }
    }

    const output = new Set<string>();
    paths.forEach((p) =>
      output.add(util.positionString(parseReindeerString(p).position)),
    );
    this.printReindeerPath(output);
    return [minDistance, output.size + 1];
  }

  printReindeerPath(path: Set<string>) {
    for (let y = 0; y < this.height; y++) {
      let row = "";
      for (let x = 0; x < this.width; x++) {
        const currentString = util.positionString({ x, y });
        if (path.has(currentString)) {
          row += "O";
        } else if (this.map.has(currentString)) {
          row += ".";
        } else {
          row += "#";
        }
      }
      console.log(row);
    }
  }
}

const input = util.readInput("16", true);

const maze = Maze.fromInput(input);

util.writeOutput(...maze.lowestCost());
