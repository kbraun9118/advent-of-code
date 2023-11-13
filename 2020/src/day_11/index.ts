import * as lib from '../lib';

enum Seat {
  Empty = 'L',
  Occupied = '#',
  Floor = '.',

}

class Ferry {

  private layout: Seat[][];

  get maxX(): number {
    return this.layout[0].length - 1;
  }

  get maxY(): number {
    return this.layout.length - 1;
  }

  constructor(input: string[] | Seat[][]) {
    if (typeof input[0] === 'string') {
      // @ts-ignore
      this.layout = input.map(inner => inner.split('').map(seat => seat as Seat));
    } else {
      // @ts-ignore
      this.layout = input;
    }
  }

  clone(): Ferry {
    return new Ferry(this.layout.map(inner => inner.map(seat => seat)));
  }

  countOccupiedNeighbors(y: number, x: number): number {
    let count = 0;
    x > 0 && y > 0 && this.layout[y - 1][x - 1] === Seat.Occupied && count++;
    y > 0 && this.layout[y - 1][x] === Seat.Occupied && count++;
    y > 0 && x < this.maxX && this.layout[y - 1][x + 1] === Seat.Occupied && count++;
    x < this.maxX && this.layout[y][x + 1] === Seat.Occupied && count++;
    y < this.maxY && x < this.maxX && this.layout[y + 1][x + 1] === Seat.Occupied && count++;
    y < this.maxY && this.layout[y + 1][x] === Seat.Occupied && count++;
    y < this.maxY && x > 0 && this.layout[y + 1][x - 1] === Seat.Occupied && count++;
    x > 0 && this.layout[y][x - 1] === Seat.Occupied && count++;

    return count;
  }


  /**
   * 0 1 2
   * 7 X 3
   * 6 5 4
   */
  private traverseDirection(y: number, x: number, direction: number): number {
    let [xDiff, yDiff] = [0, 0];
    while (true) {
      if (direction < 3) {
        yDiff--;
      } else if (direction > 3 && direction < 7) {
        yDiff++;
      }
      if (direction > 1 && direction < 5) {
        xDiff++;
      } else if (direction === 0 || direction > 5) {
        xDiff--;
      }
      if (x + xDiff < 0 || x + xDiff > this.maxX || y + yDiff < 0 || y + yDiff > this.maxY) {
        return 0;
      }
      const seat = this.layout[y + yDiff][x + xDiff];
      if (seat === Seat.Occupied) {
        return 1;
      } else if (seat === Seat.Empty) {
        return 0;
      }
    }
  }

  countOccupiedNeighbors2(y: number, x: number): number {
    let count = 0;
    for (let i = 0; i < 8; i++) {
      count += this.traverseDirection(y, x, i);
    }
    return count;
  }

  tick(maxNeighbors: number, part1: boolean = true): Ferry {
    return new Ferry(
      this.layout.map((line, y) => line.map((seat, x) => {
        if (seat === Seat.Floor) {
          return Seat.Floor;
        } else {
          const neighbors = part1 ? this.countOccupiedNeighbors(y, x) : this.countOccupiedNeighbors2(y, x);
          if (seat === Seat.Empty && neighbors === 0) {
            return Seat.Occupied;
          } else if (seat === Seat.Occupied && neighbors >= maxNeighbors) {
            return Seat.Empty;
          } else {
            return seat;
          }
        }
      })),
    );
  }

  countOccupied(): number {
    return this.layout.reduce<number>(
      (acc, next) => acc + next.reduce<number>(
        (innerAcc, seat) => innerAcc + (seat === Seat.Occupied ? 1 : 0),
        0,
      ),
      0,
    );
  }

  equals(other: Ferry): boolean {
    return this.layout.reduce<boolean>(
      (acc, next, y) => acc && next.reduce<boolean>(
        (innerAcc, seat, x) => innerAcc && seat === other.layout[y][x],
        true,
      ),
      true,
    );
  }

  print() {
    this.layout.forEach(line => console.log(line.join('')));
  }
}

const lines = lib.readLines(__dirname + '/input.txt');

let previous = new Ferry(lines);
let next = previous.tick(4);

while (!previous.equals(next)) {
  previous = next;
  next = next.tick(4);
}

lib.writePart1(next.countOccupied());


previous = new Ferry(lines);
next = previous.tick(5, false);

while (!previous.equals(next)) {
  previous = next;
  next = next.tick(5, false);
}

lib.writePart2(next.countOccupied());
