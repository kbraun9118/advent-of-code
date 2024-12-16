import util, { Position } from "../util";

class ClawMachine {
  constructor(
    private buttonA: Position,
    private buttonB: Position,
    private prize: Position,
  ) {}

  static fromLines(input: string[]): ClawMachine {
    const buttonRegex = /Button [A|B]: X\+(\d+), Y\+(\d+)/;
    const buttonAMatch = input[0].match(buttonRegex)!;
    const buttonA = { x: +buttonAMatch[1], y: +buttonAMatch[2] };
    const buttonBMatch = input[1].match(buttonRegex)!;
    const buttonB = { x: +buttonBMatch[1], y: +buttonBMatch[2] };
    const prizeMatch = input[2].match(/Prize: X=(\d+), Y=(\d+)/)!;
    const prize = { x: +prizeMatch[1], y: +prizeMatch[2] };
    return new ClawMachine(buttonA, buttonB, prize);
  }

  minCost(prizeAddition: number = 0): number | undefined {
    const prize = {
      x: this.prize.x + prizeAddition,
      y: this.prize.y + prizeAddition,
    };
    const nB =
      (this.buttonA.x * prize.y - this.buttonA.y * prize.x) /
      (this.buttonB.y * this.buttonA.x - this.buttonB.x * this.buttonA.y);
    const remB =
      (this.buttonA.x * prize.y - this.buttonA.y * prize.x) %
      (this.buttonB.y * this.buttonA.x - this.buttonB.x * this.buttonA.y);
    const nA = (prize.x - nB * this.buttonB.x) / this.buttonA.x;
    const remA = (prize.x - nB * this.buttonB.x) % this.buttonA.x;

    if (remB !== 0 || remA !== 0) {
      return undefined;
    }

    return nA * 3 + nB;
  }
}

const input = util.readInput("13");

const machines = util
  .batchWhile(input, (item) => item !== "")
  .map((l) => ClawMachine.fromLines(l));

const part1 = machines
  .map((m) => m.minCost())
  .filter((c) => c)
  .reduce<number>((acc, next) => acc + next!, 0);

const part2 = machines
  .map((m) => m.minCost(10_000_000_000_000))
  .filter((c) => c)
  .reduce<number>((acc, next) => acc + next!, 0);

util.writeOutput(part1, part2);
