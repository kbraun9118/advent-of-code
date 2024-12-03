import util from "../util";

const mulPattern = /((mul\((\d{1,3}),(\d{1,3})\))|(do\(\))|(don't\(\)))/g;

function sumProgram(line: string, enableDisable: boolean): number {
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
      sum += +match[3] * +match[4];
    }
  }

return sum;
}

const input = util.readInput("03").join();

const part1 = sumProgram(input, false)
const part2 = sumProgram(input, true)

util.writeOutput(part1, part2);
