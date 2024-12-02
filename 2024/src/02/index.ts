import util from "../util";

function isValid(level: number[]): boolean {
  if (level[0] === level[1]) {
    return false;
  }

  const isIncreasing = level[0] - level[1] < 0;

  for (let i = 0; i < level.length - 1; i++) {
    const diff = level[i] - level[i + 1];
    // stays increasing or decreasing
    if ((isIncreasing && diff > 0) || (!isIncreasing && diff < 0)) {
      return false;
    }

    // is 1, 2, or 3
    if (Math.abs(diff) < 1 || Math.abs(diff) > 3) {
      return false;
    }
  }

  return true;
}

function isValidWithRemovable(level: number[]): boolean {
  if (level[0] === level[1])
}

const input = util.readInput("02");

const levels = input.map((level) => level.split(" ").map((report) => +report));
const part1 = levels.filter(isValid).length;

util.writeOutput(part1);
