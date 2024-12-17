import util, { Position } from "../util";

type Robot = {
  position: Position;
  vector: Position;
};

function parseRobot(line: string): Robot {
  const match = line.match(/p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)/)!;

  return {
    position: { x: +match[1], y: +match[2] },
    vector: { x: +match[3], y: +match[4] },
  };
}

class RestRoom {
  height: number;
  width: number;
  map: Robot[];

  constructor(private isTest: boolean = false) {
    this.height = isTest ? 7 : 103;
    this.width = isTest ? 11 : 101;
    this.map = [];
  }

  addRobot(robot: Robot): RestRoom {
    this.map.push(robot);
    return this;
  }

  private moveRobot(robot: Robot): Robot {
    const nextPosition = {
      x: (robot.position.x + robot.vector.x) % this.width,
      y: (robot.position.y + robot.vector.y) % this.height,
    };
    if (nextPosition.x < 0) {
      nextPosition.x = this.width + nextPosition.x;
    }
    if (nextPosition.y < 0) {
      nextPosition.y = this.height + nextPosition.y;
    }
    return { position: nextPosition, vector: robot.vector };
  }

  tick(): RestRoom {
    const next = new RestRoom(this.isTest);

    for (const robot of this.map) {
      next.addRobot(this.moveRobot(robot));
    }

    return next;
  }

  quadrants(): Robot[][] {
    const midX = Math.floor(this.width / 2);
    const midY = Math.floor(this.height / 2);
    const quads: Robot[][] = [[], [], [], []];
    for (const robot of this.map) {
      if (robot.position.x < midX) {
        if (robot.position.y < midY) {
          quads[0].push(robot);
        } else if (robot.position.y > midY) {
          quads[1].push(robot);
        }
      } else if (robot.position.x > midX) {
        if (robot.position.y < midY) {
          quads[2].push(robot);
        } else if (robot.position.y > midY) {
          quads[3].push(robot);
        }
      }
    }
    return quads;
  }

  robotCount(position: Position): number {
    return this.map.filter(
      (r) => r.position.x === position.x && r.position.y === position.y,
    ).length;
  }

  neighbors(position: Position): Robot[] {
    const neighborPositions = [
      { x: 0, y: 1 },
      { x: 0, y: -1 },
      { x: -1, y: 0 },
      { x: 1, y: 0 },
    ].map((p) => ({ x: position.x + p.x, y: position.y + p.y }));
    return this.map.filter((r) =>
      neighborPositions.some(
        (np) => r.position.x === np.x && r.position.y === np.y,
      ),
    );
  }

  allHaveNeighbors(): boolean {
    for (const robot of this.map) {
      if (this.neighbors(robot.position).length === 0) {
        return false
      }
    }
    return true;
  }

  noOverlap(): boolean {
    for (const robot of this.map) {
      if (this.robotCount(robot.position) > 1) {
        return false;
      }
    }
    return true;
  }

  printMap(): void {
    for (let y = 0; y < this.height; y++) {
      let line = "";
      for (let x = 0; x < this.width; x++) {
        if (this.robotCount({ x, y }) > 0) {
          line += "X";
        } else {
          line += " ";
        }
      }
      console.log(line);
    }
  }
}

function part1(restRoom: RestRoom): number {
  let current = restRoom;
  for (let i = 0; i < 100; i++) {
    current = current.tick();
  }

  return current
    .quadrants()
    .map((r) => r.length)
    .reduce((acc, next) => acc * next);
}

function part2(restRoom: RestRoom): number {
  let current = restRoom;
  let ticks = 0;
  while (!current.noOverlap()) {
    current = current.tick();
    ticks++;
  }
  return ticks;
}

const input = util.readInput("14");

const restRoom = input
  .map((l) => parseRobot(l))
  .reduce((rr, next) => rr.addRobot(next), new RestRoom());

util.writeOutput(part1(restRoom), part2(restRoom));
