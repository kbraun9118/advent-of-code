import util from "../util";

const mulPattern = /mul\(\d{1,3},\d{1,3}\)/;

function sumForLine(line: string): number {
  const matches =  mulPattern.exec(line);

  if (!matches) {
    return 0;
  }
  let sum = 0;

  for (const match of matches) {
    console.log(match)
    sum += 1;
  }

return sum;
}

const input = util.readInput("03", true);

const part1 = input.map(sumForLine).reduce((acc, curr) => acc + curr);

util.writeOutput(part1);
