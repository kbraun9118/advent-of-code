import * as lib from '../lib';

const setDirection = (current: number, direction: string, amount: number): number => {

  const value = (current + (((direction === 'R' ? 1 : -1) * amount) / 90)) % 4;
  if (value < 0) {
    return 4 + value;
  } else {
    return value;
  }
};
const moveDirection = (x: number, y: number, direction: string, amount: number): [number, number] => {

  switch (direction) {
    case 'E':
      return [x + amount, y];
    case 'S':
      return [x, y - amount];
    case 'W':
      return [x - amount, y];
    case 'N':
      return [x, y + amount];
    default:
      return [x, y];
  }
};

const lines = lib.readLines(__dirname + '/input.txt');
const directions = lines.map(line => {
  const instruction = line.substring(0, 1);
  const value = line.substring(1);
  return {instruction, value}
})

let [x, y, direction] = [0, 0, 0];

directions.forEach(({ instruction, value }) => {
  if (instruction === 'L' || instruction === 'R') {
    direction = setDirection(direction, instruction, +value);
  } else {
    if (instruction === 'F') {
      switch (direction) {
        case 0:
          instruction = 'E';
          break;
        case 1:
          instruction = 'S';
          break;
        case 2:
          instruction = 'W';
          break;
        case 3:
          instruction = 'N';
          break;
      }
    }
    [x, y] = moveDirection(x, y, instruction, +value);
  }
});

lib.writePart1(Math.abs(x) + Math.abs(y));

const rotateWayPoint = (x: number, y: number, direction: string, deg: number): [number, number] => {
  let direc = direction;
  if (deg === 180) {
    return [-1 * x, -1 * y];
  } else if (deg === 270) {
    direc = direc === 'L' ? 'R' : 'L';
  }
  if (direc === 'L') {
      return [-1 * y, x];
  } else {
    return [y, -x];
  }
}

const moveWayPoint = (x: number, y: number, direction: string, distance: number): [number, number] => {
  switch (direction) {
    case 'N':
      return [x, y + distance];
    case 'S':
      return [x, y - distance];
    case 'E':
      return [x + distance, y];
    case 'W':
      return [x - distance, y];
    default:
      return [x, y];
  }
}

const moveShip = (x: number, y: number, wayX: number, wayY: number, distance: number): [number, number] => {
  return [x + (wayX * distance), y + (wayY * distance)];
}

[x, y] = [0, 0];
let [wayX, wayY] = [10, 1];

directions.forEach(({ instruction, value }) => {
  if (instruction === 'F') {
    [x, y] = moveShip(x, y, wayX, wayY, +value);
  } else if (instruction === 'L' || instruction === 'R') {
    [wayX, wayY] = rotateWayPoint(wayX, wayY, instruction, +value);
  } else {
    [wayX, wayY] = moveWayPoint(wayX, wayY, instruction, +value);
  }
});

lib.writePart2(Math.abs(x) + Math.abs(y));
