import util, { Position } from "../util";

const nextPosition: Record<string, Position> = {
  "^": { x: 0, y: -1 },
  v: { x: 0, y: 1 },
  ">": { x: 1, y: 0 },
  "<": { x: -1, y: 0 },
};

class Warehouse {
  map: string[][];
  robot: Position;
  private enlarged: boolean = false;

  constructor(map: string[]) {
    this.map = map.map((l) => l.split(""));
    this.robot = this.map
      .flatMap((l, y) => l.map((p, x) => ({ p, x, y })))
      .find(({ p }) => p === "@")!;
  }

  enlarge() {
    this.map = this.map.map((r) =>
      r
        .map((p) => {
          if (p === "#") {
            return ["#", "#"];
          }
          if (p === "O") {
            return ["[", "]"];
          }
          if (p === ".") {
            return [".", "."];
          }
          return ["@", "."];
        })
        .flat(),
    );
    this.robot = this.map
      .flatMap((l, y) => l.map((p, x) => ({ p, x, y })))
      .find(({ p }) => p === "@")!;
    this.enlarged = true;
  }

  private moveRock(rock: Position, direction: string): void {
    if (this.map[rock.y][rock.x] !== "O") {
      return;
    }

    const nextPos = {
      x: rock.x + nextPosition[direction].x,
      y: rock.y + nextPosition[direction].y,
    };
    this.moveRock(nextPos, direction);

    if (this.map[nextPos.y][nextPos.x] === ".") {
      this.map[nextPos.y][nextPos.x] = "O";
      this.map[rock.y][rock.x] = ".";
    }
  }

  private canMoveLargeRock(rock: Position, direction: string): boolean {
    const rockValue = this.map[rock.y][rock.x];
    if (rockValue === ".") {
      return true;
    }
    if (rockValue === "#") {
      return false;
    }
    const nextPos = {
      x: rock.x + nextPosition[direction].x,
      y: rock.y + nextPosition[direction].y,
    };
    if (direction === "v" || direction === "^") {
      if (rockValue === "[") {
        return (
          this.canMoveLargeRock(nextPos, direction) &&
          this.canMoveLargeRock({ ...nextPos, x: nextPos.x + 1 }, direction)
        );
      }
      return (
        this.canMoveLargeRock(nextPos, direction) &&
        this.canMoveLargeRock({ ...nextPos, x: nextPos.x - 1 }, direction)
      );
    }

    return this.canMoveLargeRock(
      { ...nextPos, x: nextPos.x + nextPosition[direction].x },
      direction,
    );
  }

  private moveLargeRock(rock: Position, direction: string): void {
    const rockValue = this.map[rock.y][rock.x];
    if (rockValue !== "[" && rockValue !== "]") {
      return;
    }
    const nextPos = {
      x: rock.x + nextPosition[direction].x,
      y: rock.y + nextPosition[direction].y,
    };
    this.map[rock.y][rock.x] = ".";
    if (direction === "v" || direction === "^") {
      this.moveLargeRock(nextPos, direction);
      if (rockValue === "[") {
        this.moveLargeRock({ ...nextPos, x: nextPos.x + 1 }, direction);
        this.map[rock.y][rock.x + 1] = ".";
        this.map[nextPos.y][nextPos.x] = "[";
        this.map[nextPos.y][nextPos.x + 1] = "]";
        return;
      }
      this.map[rock.y][rock.x - 1] = ".";
      this.moveLargeRock({ ...nextPos, x: nextPos.x - 1 }, direction);
      this.map[nextPos.y][nextPos.x] = "]";
      this.map[nextPos.y][nextPos.x - 1] = "[";
      return;
    }
    const nextNextPos = {
      x: nextPos.x + nextPosition[direction].x,
      y: nextPos.y + nextPosition[direction].y,
    };
    if (direction === ">") {
      this.moveLargeRock(nextNextPos, direction);
      this.map[nextNextPos.y][nextNextPos.x] = "]";
      this.map[nextPos.y][nextPos.x] = "[";
      return;
    }

    this.moveLargeRock(nextNextPos, direction);
    this.map[nextNextPos.y][nextNextPos.x] = "[";
    this.map[nextPos.y][nextPos.x] = "]";
  }

  moveRobot(direction: string): void {
    const nextPos = {
      x: this.robot.x + nextPosition[direction].x,
      y: this.robot.y + nextPosition[direction].y,
    };

    if (this.enlarged && this.canMoveLargeRock(nextPos, direction)) {
      this.moveLargeRock(nextPos, direction);
    } else {
      this.moveRock(nextPos, direction);
    }

    if (this.map[nextPos.y][nextPos.x] === ".") {
      this.map[nextPos.y][nextPos.x] = "@";
      this.map[this.robot.y][this.robot.x] = ".";
      this.robot = nextPos;
    }
  }

  gpsSum(): number {
    return this.map
      .flatMap((r, y) => r.map((p, x) => ({ p, x, y })))
      .filter(({ p }) => p === "O" || p === "[")
      .reduce((acc, next) => acc + next.y * 100 + next.x, 0);
  }

  printMap(): void {
    for (const row of this.map) {
      console.log(row.join(""));
    }
  }
}

function calcPart1(warehouse: Warehouse, directions: string[]): number {
  for (const direction of directions) {
    warehouse.moveRobot(direction);
  }
  return warehouse.gpsSum();
}

function calcPart2(warehouse: Warehouse, directions: string[]): number {
  warehouse.enlarge();
  for (const direction of directions) {
    warehouse.moveRobot(direction);
  }
  return warehouse.gpsSum();
}

const input = util.readInput("15");

const splitIndex = input.indexOf("");
const mapString = input.slice(0, splitIndex);
const directions = input.slice(splitIndex + 1).flatMap((l) => l.split(""));

let warehouse = new Warehouse(mapString);
const part1 = calcPart1(warehouse, directions);

warehouse = new Warehouse(mapString);
const part2 = calcPart2(warehouse, directions);

util.writeOutput(part1, part2);
