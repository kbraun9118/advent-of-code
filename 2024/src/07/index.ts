import util from "../util";

function containsTarget(
  equation: number[],
  target: number,
  useConcate: boolean = false,
): boolean {
  let previous = [equation[0]];
  for (let i = 1; i < equation.length; i++) {
    const current = [];
    for (let previousNum of previous) {
      const addition = previousNum + equation[i];
      const multiplicaiton = previousNum * equation[i];
      if (addition <= target) {
        current.push(addition);
      }
      if (multiplicaiton <= target) {
        current.push(multiplicaiton);
      }
      if (useConcate) {
        const concatenation = +(`${previousNum}` + equation[i]);
        if (concatenation <= target) {
          current.push(concatenation);
        }
      }
    }
    previous = current;
  }

  return previous.includes(target);
}

const input: [number, number[]][] = util
  .readInput("07")
  .map((line) => line.split(": "))
  .map(([target, equation]) => [+target, equation.split(" ").map((e) => +e)]);

const part1 = input
  .filter(([target, equation]) => containsTarget(equation, target))
  .reduce((acc, [target]) => acc + target, 0);
const part2 = input
  .filter(([target, equation]) => containsTarget(equation, target, true))
  .reduce((acc, [target]) => acc + target, 0);

util.writeOutput(part1, part2);
