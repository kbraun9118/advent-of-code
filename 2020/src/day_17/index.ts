import * as lib from '../lib';

type Coordinate3D = { z: number; y: number; x: number; };

class Dimension3D {
  active: { [z: number]: { [y: number]: { [x: number]: true } } };

  constructor() {
    this.active = {};
  }

  push({ z, y, x }: Coordinate3D) {
    if (this.active[z]) {
      if (this.active[z][y]) {
        this.active[z][y][x] = true;
        return;
      }
    }
    this.active[z] = {
      ...this.active[z],
      [y]: {
        [x]: true,
      },
    };
  }

  isActive({ z, y, x }: Coordinate3D): boolean {
    if (this.active[z] && this.active[z][y]) {
      return this.active[z][y][x] || false;
    } else {
      return false;
    }
  }

  get coordinates(): Coordinate3D[] {
    const coordinates: Coordinate3D[] = [];
    Object.entries(this.active)
      .forEach(([z, ys]) => Object.entries(ys)
        .forEach(([y, xs]) => Object.keys(xs)
          .forEach(x => coordinates.push({
            x: +x,
            y: +y,
            z: +z,
          })),
        ),
      );
    return coordinates;
  }

  static neighbors({ z, y, x }: Coordinate3D): Coordinate3D[] {
    let coordinates: Coordinate3D[] = [];
    for (let i = -1; i < 2; i++) {
      for (let j = -1; j < 2; j++) {
        for (let k = -1; k < 2; k++) {
          coordinates.push({
            z: z + i,
            y: y + j,
            x: x + k,
          });
        }
      }
    }
    return coordinates.filter((coordinate) => coordinate.x !== x || coordinate.y !== y || coordinate.z !== z);
  }

  next(): Dimension3D {
    const next = new Dimension3D();
    this.coordinates.forEach(coord => {
      let neighbors = Dimension3D.neighbors(coord);
      neighbors
        .filter(neighbor => !this.isActive(neighbor) && Dimension3D.neighbors(neighbor)
          .filter(neighborsNeighbor => this.isActive(neighborsNeighbor))
          .length === 3)
        .forEach(neighbor => next.push(neighbor));
      const neighborsCount = neighbors.filter(neighbor => this.isActive(neighbor)).length;
      if (neighborsCount === 2 || neighborsCount === 3) {
        next.push(coord);
      }
    });

    return next;
  }
}

type Coordinate4D = { z: number; y: number; x: number; t: number };

class Dimension4D {
  active: { [z: number]: { [y: number]: { [x: number]: { [t: number]: true } } } };

  constructor() {
    this.active = {};
  }

  push({ z, y, x, t }: Coordinate4D) {
    if (this.active[z]) {
      if (this.active[z][y]) {
        if (this.active[z][y][x]) {
          this.active[z][y][x][t] = true;
        } else {
          this.active[z][y][x] = { [t]: true}
        }
      } else {
        this.active[z][y] = { [x]: { [t]: true } };
      }
    } else {
      this.active[z] = { [y]: { [x]: { [t]: true } } };
    }
  }

  isActive({ z, y, x, t }: Coordinate4D): boolean {
    if (this.active[z] && this.active[z][y] && this.active[z][y][x]) {
      return this.active[z][y][x][t] || false;
    } else {
      return false;
    }
  }

  get coordinates(): Coordinate4D[] {
    const coordinates: Coordinate4D[] = [];
    Object.entries(this.active)
      .forEach(([z, ys]) => Object.entries(ys)
        .forEach(([y, xs]) => Object.entries(xs)
          .forEach(([x, ts]) => Object.keys(ts)
            .forEach(t => coordinates.push({
              x: +x,
              y: +y,
              z: +z,
              t: +t,
            })),
          ),
        ),
      );
    return coordinates;
  }

  static neighbors({ z, y, x, t }: Coordinate4D): Coordinate4D[] {
    let coordinates: Coordinate4D[] = [];
    for (let i = -1; i < 2; i++) {
      for (let j = -1; j < 2; j++) {
        for (let k = -1; k < 2; k++) {
          for (let l = -1; l < 2; l++) {
            coordinates.push({
              z: z + i,
              y: y + j,
              x: x + k,
              t: t + l,
            });
          }
        }
      }
    }
    return coordinates.filter((coordinate) => coordinate.x !== x || coordinate.y !== y || coordinate.z !== z || coordinate.t !== t);
  }

  next(): Dimension4D {
    const next = new Dimension4D();
    this.coordinates.forEach(coord => {
      let neighbors = Dimension4D.neighbors(coord);
      neighbors
        .filter(neighbor => !this.isActive(neighbor) && Dimension4D.neighbors(neighbor)
          .filter(neighborsNeighbor => this.isActive(neighborsNeighbor))
          .length === 3)
        .forEach(neighbor => next.push(neighbor));
      const neighborsCount = neighbors.filter(neighbor => this.isActive(neighbor)).length;
      if (neighborsCount === 2 || neighborsCount === 3) {
        next.push(coord);
      }
    });

    return next;
  }
}

const lines = lib.readLines('17');

let dimension3D = new Dimension3D();
let dimension4D = new Dimension4D();

lines.forEach((line, y) => line.split('').forEach((char, x) => {
  if (char === '#') {
    dimension3D.push({ z: 0, x, y });
    dimension4D.push({ z: 0, t: 0, x, y });
  }
}));

for (let i = 0; i < 6; i++) {
  dimension3D = dimension3D.next();
  dimension4D = dimension4D.next();
}

lib.writePart1(dimension3D.coordinates.length);
lib.writePart2(dimension4D.coordinates.length);
