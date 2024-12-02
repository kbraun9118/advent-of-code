import util from "../util";

const input = util.readInput("01");

const [left, right] = input
  .map((line) => line.split("   "))
  .reduce(
    ([prevL, prevR], [left, right]) => {
      prevL.push(+left);
      prevR.push(+right);
      return [prevL, prevR];
    },
    [[] as number[], [] as number[]],
  );
left.sort();
right.sort();
let part1 = 0;

for (let i = 0; i < left.length; i++) {
  part1 += Math.abs(left[i] - right[i]);
}

const occuranceMap: { [key: number]: number } = {};

for (const key of right) {
  if (occuranceMap[key]) {
    occuranceMap[key] += 1;
  } else {
    occuranceMap[key] = 1;
  }
}

let part2 = 0;

for (const key of left) {
  if (occuranceMap[key]) {
    part2 += key * occuranceMap[key];
  }
}

util.writeOutput(part1, part2);
