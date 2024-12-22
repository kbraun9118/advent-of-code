import { MinHeap } from "@datastructures-js/heap";
import util, { HashMap, HashSet, Position } from "../util";

type Direction = "N" | "E" | "W" | "S";

type Reindeer = {
  position: Position;
  facing: Direction;
};

type ReindeerCost = {
  reindeer: Reindeer;
  cost: number;
};


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

  move(
    source: ReindeerCost,
    destination: Position,
  ): ReindeerCost {
    const positionDiff = {
      x: source.reindeer.position.x - destination.x,
      y: source.reindeer.position.y - destination.y,
    };
    if (source.reindeer.facing === "N" || source.reindeer.facing === "S") {
      if (positionDiff.x === 0) {
        return {
          cost: source.cost + 1,
          reindeer: {
            position: destination,
            facing: source.reindeer.facing,
          },
        };
      }

      return {
        cost: source.cost + 1001,
        reindeer: {
          position: destination,
          facing: positionDiff.x < 0 ? "E" : "W",
        },
      };
    }

    if (positionDiff.y === 0) {
      return {
        cost: source.cost + 1,
        reindeer: {
          position: destination,
          facing: source.reindeer.facing,
        },
      };
    }
    return {
      cost: source.cost + 1001,
      reindeer: {
        position: destination,
        facing: positionDiff.y < 0 ? "S" : "N",
      },
    };
  }

  lowestCost(): [number, number] {
    const prev = new HashMap<Reindeer, ReindeerCost[]>();
    const dist = new HashMap<Reindeer, number>();
    const queue = new MinHeap<ReindeerCost>((r) => r.cost);
    const startReindeer: ReindeerCost = {
      cost: 0,
      reindeer: {
        position: this.start,
        facing: "E",
      },
    };
    queue.push(startReindeer);
    prev.set(startReindeer.reindeer, []);

    while (queue.size() > 0) {
      const current = queue.pop()!;
      if (
        current.reindeer.position.x === this.end.x &&
        current.reindeer.position.y === this.end.y
      ) {
        continue;
      }

      for (const neighbor of this.neighbors(current.reindeer.position)) {
        const next = this.move(current, neighbor);
        if (dist.has(next.reindeer)) {
          const nextDist = dist.get(next.reindeer)!;
          if (nextDist === next.cost) {
            prev.get(next.reindeer)?.push(current);
          }
          continue;
        } else {
          prev.set(next.reindeer, [current]);
          dist.set(next.reindeer, next.cost);
        }

        queue.push(next);
      }
    }

    const paths = new HashSet<Reindeer>();
    const endReindeer: Reindeer[] = (["N", "E", "W", "S"] as Direction[]).map(
      (d) => ({ position: this.end, facing: d }),
    );
    const minDistance = endReindeer
      .map((r) => dist.get(r))
      .filter((r) => r !== undefined)
      .reduce((acc, next) => Math.min(acc, next));
    const pathQueue = endReindeer.filter(
      (r) => dist.get(r) === minDistance,
    );

    while (pathQueue.length > 0) {
      const current = pathQueue.shift()!;
      const prevPaths = prev.get(current)!;
      if (
        current.position.x === this.start.x &&
        current.position.y === this.start.y
      ) {
        continue;
      }
      for (const path of prevPaths) {
        if (paths.has(path.reindeer)) {
          continue;
        }
        paths.add(path.reindeer);
        pathQueue.push(path.reindeer);
      }
    }

    const output = new Set<string>();
    paths.forEach((p) =>
      output.add(util.positionString(p.position)),
    );
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

const input = util.readInput("16");

const maze = Maze.fromInput(input);

util.writeOutput(...maze.lowestCost());
