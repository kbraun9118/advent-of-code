import util from "../util";

const mulPattern = /(mul\((\d{1,3}),(\d{1,3})\))|(do\(\))|(don't\(\))/g;

function sumForLine(line: string, enableDisable: boolean): number {
  let matches = line.matchAll(mulPattern);

  let sum = 0;
  let enabled = true;

  for (const match of matches) {

    if (enableDisable) {
      if (match[0].startsWith("don't")) {
        enabled = false;
      }
      if (match[0].startsWith("do(")) {
        enabled = true;
      }
    }

    if (enabled && match[0].startsWith("mul")) {
      sum += +match[2] * +match[3];
    }
  }

return sum;
}

const input = util.readInput("03");

const part1 = input.map(line => sumForLine(line, false)).reduce((acc, curr) => acc + curr);
const part2 = input.map(line => sumForLine(line, true)).reduce((acc, curr) => acc + curr);

util.writeOutput(part1, part2);
