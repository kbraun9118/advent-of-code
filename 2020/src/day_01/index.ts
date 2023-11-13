import * as lib from '../lib';

const lines = lib.readLines(`${__dirname}/input.txt`);

let first: number = 0;
let second: number = 0;

for (let i = 0; i < lines.length - 1; i++) {
  for (let j = i + 1; j < lines.length; j++) {
    if (+lines[i] + +lines[j] === 2020) {
      first = +lines[i];
      second = +lines[j];
      break;
    }
  }
}

if (first !== 0 && second !== 0) {
  console.log('Part 1: ' + first * second);
} else {
  console.log('error');
}

first = 0;
second = 0;
let third = 0;

for (let i = 0; i < lines.length - 2; i++) {
  for (let j = i + 1; j < lines.length - 1; j++) {
    for (let k = j + 1; k < lines.length; k++) {
      if (+lines[i] + +lines[j] + +lines[k] === 2020) {
        first = +lines[i];
        second = +lines[j];
        third = +lines[k];
        break;
      }
    }
  }
}

if (first !== 0 && second !== 0 && third !== 0) {
  console.log('Part 2: ' + first * second * third);
} else {
  console.log('error');
}
