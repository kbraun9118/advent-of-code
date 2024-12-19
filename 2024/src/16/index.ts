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

class Maze {
  map: Set<string> = new Set();
  start: Position = { x: 0, y: 0 };
  end: Position = { x: 0, y: 0 };

  private constructor() {}

  static fromInput(input: string[]): Maze {
    const maze = new Maze();

    for (const [y, row] of input.entries()) {
      for (const [x, v] of row.split("").entries()) {
        if (v === "." || v === "E" || v === "S") {
          maze.map.add(util.positionString({ x, y }));
          if (v === "S") {
            maze.start = { x, y };
          }
          if (v === "E") {
            maze.end = { x, y };
          }
        }
      }
    }

    return maze;
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
        facing: positionDiff.x < 0 ? "W" : "E",
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
      facing: positionDiff.y < 0 ? "N" : "S",
    };
  }

  lowestCost(): number {
    // const dist: Map<string, number> = new Map(
    //   Array.from(this.map.values()).map((ps) => [ps, Number.POSITIVE_INFINITY]),
    // );
    // dist.set(util.positionString(this.start), 0);
    const visited = new Set<string>();
    const queue = new MinHeap<Reindeer>((r) => r.cost);
    queue.push({ position: this.start, cost: 0, facing: "E" });

    while (queue.size() > 0) {
      const current = queue.pop()!;
      if (
        current.position.x === this.end.x &&
        current.position.y === this.end.y
      ) {
        return current.cost;
      }

      for (const neighbor of this.neighbors(current.position)) {
        const next = this.move(current, neighbor);
        if (visited.has(reindeerString(next))) {
          continue;
        }
        
        visited.add(reindeerString(next)) ;
        queue.push(next);
      }
    }

    return Number.POSITIVE_INFINITY;
  }
}

const input = util.readInput("16");

const maze = Maze.fromInput(input);


util.writeOutput(maze.lowestCost());
