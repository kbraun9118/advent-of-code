import * as lib from '../lib';

const [input] = lib.readLines(__dirname + '/input.txt').map(line => line.split(',').map(char => +char));

const solve = (starting: number[], part1 = true): number => {
  let sequence = [...starting];
  const dictionary: Map<number, number> = new Map<number, number>();
  sequence.slice(0, sequence.length - 1).forEach((num, i) => dictionary.set(num, i));
  // console.log(dictionary);
  let nextToBeInserted = starting[sequence.length - 1];
  for (let i = sequence.length; i < (part1 ? 2020 : 30000000); i++) {
    // console.log(nextToBeInserted);
    if (dictionary.has(nextToBeInserted)) {
      const diff = i - dictionary.get(nextToBeInserted)!! - 1;
      dictionary.set(nextToBeInserted, i - 1);
      nextToBeInserted = diff;
    } else {
      // console.log('not found');
      dictionary.set(nextToBeInserted, i - 1);
      nextToBeInserted = 0;
    }
    // console.log(dictionary);
  }
  return nextToBeInserted;
};

lib.writePart1(solve(input));
lib.writePart2(solve(input, false));
