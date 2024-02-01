import * as lib from '../lib';

const lines = lib.readLines('10');

const sorted = lines.map(line => +line).sort((left, right) => left - right);

let outlet = sorted[sorted.length - 1] + 3;
const jolts = [0, ...sorted, outlet];

const countDifferences = (list: number[], diff: number): number => {
  let count = 0;
  for (let i = 0; i < list.length - 1; i++) {
    list[i + 1] - list[i] === diff && count++;
  }
  return count;
};

lib.writePart1(countDifferences(jolts, 1) * countDifferences(jolts, 3));

const countCombinations = (list: number[], upper: number, table: {[count:number]: number} = {}): number => {
  const diff1 = table[upper - 1] || (upper - 1 === 0 ? 1 : !list.find(item => item === upper - 1) ? 0 : countCombinations(list, upper - 1, table));
  const diff2 = table[upper - 2] || (upper - 2 === 0 ? 1 : !list.find(item => item === upper - 2) ? 0 : countCombinations(list, upper - 2, table));
  const diff3 = table[upper - 3] || (upper - 3 === 0 ? 1 : !list.find(item => item === upper - 3) ? 0 : countCombinations(list, upper - 3, table));

  table[upper - 1] = diff1;
  table[upper - 2] = diff2;
  table[upper - 3] = diff3;

  return diff1 + diff2 + diff3;
}

lib.writePart2(countCombinations(sorted, outlet));
