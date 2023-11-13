import * as lib from '../lib';

const lines = lib.readLines(__dirname + '/input.txt').map(line => +line);

const addsUpTo = (list: number[], index: number): boolean => {
  for (let i = index - 25; i < index - 1; i++) {
    for (let j = index - 24; j < index; j++) {
      if (list[index] === list[i] + list[j]) {
        return true;
      }
    }
  }
  return false;
};

let solution1: number | undefined;

for (let i = 25; i < lines.length; i++) {
  if (!addsUpTo(lines, i)) {
    solution1 = lines[i];
    lib.writePart1(solution1);
    break;
  }
}

const sumBetween = (list: number[], lower: number, upper: number) => {
  let acc = 0;
  for (let i = lower; i <= upper; i++) {
    acc += list[i];
  }
  return acc;
}

if (solution1) {
  outer: for (let i = 0; i < lines.length - 1; i++) {
    for (let j = i + 1; j < lines.length - 1; j++) {
      const sum = sumBetween(lines, i, j);
      if (sum === solution1) {
        const solution = lines.slice(i, j + 1);
        lib.writePart2(Math.min(...solution) + Math.max(...solution))
        break outer;
      }
      if (sum > solution1) {
        break;
      }
    }
  }
}
