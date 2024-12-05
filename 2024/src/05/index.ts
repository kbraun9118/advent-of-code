import util from "../util";

class Rules {
  ruleMap: Record<number, number[]> = {};

  constructor(ruleStrings: string[]) {
    for (const rule of ruleStrings) {
      this.addRule(rule);
    }
  }

  addRule(ruleString: string) {
    const [left, right] = ruleString.split("|").map((r) => +r);

    if (this.ruleMap[left]) {
      this.ruleMap[left].push(right);
    } else {
      this.ruleMap[left] = [right];
    }
  }

  findInvalid(update: number[]): [number, number] | undefined {
    for (let i = 1; i < update.length; i++) {
      const curr = update[i];

      if (this.ruleMap[curr]) {
        const before = update.slice(0, i);
        for (const rule of this.ruleMap[curr]) {
          const beforeIdx = before.indexOf(rule);
          if (beforeIdx !== -1) {
            return [i, beforeIdx];
          }
        }
      }
    }
    return undefined;
  }

  reorderInvalid(update: number[]): number[] {
    const newUpdate = [...update];
    let isInvalid = this.findInvalid(newUpdate);

    while (isInvalid) {
      const [left, right] = isInvalid;
      const tmp = newUpdate[left];
      newUpdate[left] = newUpdate[right];
      newUpdate[right] = tmp;
      isInvalid = this.findInvalid(newUpdate);
    }

    return newUpdate;
  }
}

const input = util.readInput("05");

const splitIdx = input.indexOf("");
const ruleStrings = input.slice(0, splitIdx);
const updates = input
  .slice(splitIdx + 1)
  .map((us) => us.split(",").map((u) => +u));

const rules = new Rules(ruleStrings);

const part1 = updates
  .filter((u) => !rules.findInvalid(u))
  .map((u) => u[Math.floor(u.length / 2)])
  .reduce((acc, curr) => acc + curr);

const part2 = updates
  .filter((u) => rules.findInvalid(u))
  .map((u) => rules.reorderInvalid(u))
  .map((u) => u[Math.floor(u.length / 2)])
  .reduce((acc, curr) => acc + curr);

util.writeOutput(part1, part2);
