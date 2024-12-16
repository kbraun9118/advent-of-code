import util, { Position } from "../util";

const neighborDiffs: Position[] = [
  { x: 0, y: 1 },
  { x: 0, y: -1 },
  { x: 1, y: 0 },
  { x: -1, y: 0 },
];

const diagonalDiff: Position[] = [
  { x: 1, y: 1 },
  { x: 1, y: -1 },
  { x: -1, y: 1 },
  { x: -1, y: -1 },
];

class Garden {
  regions: Position[][];

  constructor(private map: string[]) {
    this.regions = [];

    for (let y = 0; y < map.length; y++) {
      for (let x = 0; x < map[y].length; x++) {
        if (this.regions.some((r) => r.some((p) => p.x === x && p.y === y))) {
          continue;
        }
        const current = map[y][x];
        const currentRegion: Position[] = [];
        const currentQueue: Position[] = [{ x, y }];

        while (currentQueue.length !== 0) {
          const currentPosition = currentQueue.shift()!;
          if (
            currentRegion.some(
              (p) => currentPosition.x === p.x && currentPosition.y === p.y,
            )
          ) {
            continue;
          }
          currentRegion.push(currentPosition);
          for (const neighbor of neighborDiffs.map((d) => ({
            x: currentPosition.x + d.x,
            y: currentPosition.y + d.y,
          }))) {
            if (map[neighbor.y] && map[neighbor.y][neighbor.x] === current) {
              currentQueue.push(neighbor);
            }
          }
        }
        this.regions.push(currentRegion);
      }
    }
  }

  private perimeter(regionIdx: number): number {
    const region = this.regions[regionIdx];
    let perimeter = 0;
    for (const plot of region) {
      for (const neighbor of neighborDiffs.map((nDiff) => ({
        x: plot.x + nDiff.x,
        y: plot.y + nDiff.y,
      }))) {
        if (
          !this.map[neighbor.y] ||
          !this.map[neighbor.y][neighbor.x] ||
          !region.some((p) => p.x === neighbor.x && p.y === neighbor.y)
        ) {
          perimeter++;
        }
      }
    }

    return perimeter;
  }

  private tBase(neighbors: Position[]): Position {
    if (
      neighbors[0].x !== neighbors[1].x &&
      neighbors[0].x !== neighbors[2].x &&
      neighbors[0].y !== neighbors[1].y &&
      neighbors[0].y !== neighbors[2].y
    ) {
      return neighbors[0];
    }
    if (
      neighbors[1].x !== neighbors[0].x &&
      neighbors[1].x !== neighbors[2].x &&
      neighbors[1].y !== neighbors[0].y &&
      neighbors[1].y !== neighbors[2].y
    ) {
      return neighbors[1];
    }
    return neighbors[2];
  }

  private sides(regionIdx: number): number {
    const region = this.regions[regionIdx];
    const regionValue = this.map[region[0].y][region[0].x];
    let corners = 0;
    for (const plot of region) {
      const neighbors = neighborDiffs
        .map((nDiff) => ({
          x: plot.x + nDiff.x,
          y: plot.y + nDiff.y,
        }))
        .filter((n) => this.map[n.y] && this.map[n.y][n.x] === regionValue);
      if (neighbors.length === 0) {
        return 4;
      }
      if (neighbors.length === 1) {
        corners += 2;
      }
      if (
        neighbors.length === 2 &&
        neighbors[0].x !== neighbors[1].x &&
        neighbors[0].y !== neighbors[1].y
      ) {
        if (
          this.map[neighbors[0].y][neighbors[1].x] === regionValue &&
          this.map[neighbors[1].y][neighbors[0].x] === regionValue
        ) {
          corners += 1;
        } else {
          corners += 2;
        }
      }
      const diagonalNeighbors = diagonalDiff
        .map((nDiff) => ({
          x: plot.x + nDiff.x,
          y: plot.y + nDiff.y,
        }))
        .filter((n) => this.map[n.y] && this.map[n.y][n.x] === regionValue);
      if (neighbors.length === 3) {
        const tBase = this.tBase(neighbors);
        const diagonalTBaseNeighbors = diagonalNeighbors.filter(
          (n) => n.x === tBase.x || n.y === tBase.y,
        );
        corners += 2 - diagonalTBaseNeighbors.length;
      }
      if (neighbors.length === 4) {
        corners += 4 - diagonalNeighbors.length;
      }
    }
    return corners;
  }

  fencingCost(): number {
    let fencingCost = 0;
    for (let regionIdx = 0; regionIdx < this.regions.length; regionIdx++) {
      fencingCost += this.regions[regionIdx].length * this.perimeter(regionIdx);
    }
    return fencingCost;
  }

  fencingCostBulk(): number {
    let fencingCost = 0;
    for (let regionIdx = 0; regionIdx < this.regions.length; regionIdx++) {
      fencingCost += this.regions[regionIdx].length * this.sides(regionIdx);
    }
    return fencingCost;
  }
}

const input = util.readInput("12");

const garden = new Garden(input);

util.writeOutput(garden.fencingCost(), garden.fencingCostBulk());
