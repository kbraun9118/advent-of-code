import * as lib from '../lib';

const getAmountToCopy = (lines: string[]) => {
  const amount = lines.length;
  const length = lines[0].length;
  return Math.ceil(amount * 7 / length);
};

const copyLines = (lines: string[], amount: number) => {
  return lines.map(line => {
    let returned = line;
    for (let i = 0; i < amount; i++) {
      returned = returned.replace('\r', '');
      returned += line;
    }
    return returned;
  });
};

const calculatePath = (lines: string[], down: number, right: number) => {
  let count = 0;
  for (let i = 0; i * down < lines.length; i++) {
    if (linesForLength[i * down].charAt(i * right) === '#') {
      count++;
    }
  }
  return count;
};


const lines = lib.readLines(__dirname + '/input.txt');
const linesForLength = copyLines(lines, getAmountToCopy(lines));

let part1 = calculatePath(lines, 1, 3);
console.log('Part 1: ' + part1);

const part2 = [
  part1,
  calculatePath(lines, 1, 1),
  calculatePath(lines, 1, 5),
  calculatePath(lines, 1, 7),
  calculatePath(lines, 2, 1),
];

console.log('Part 2: ' + part2.reduce((combination, next) => combination * next, 1));
