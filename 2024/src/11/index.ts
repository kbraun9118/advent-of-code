import util from "../util";

function blink(stones: number[]): number[] {
  const output: number[] = [];
  for (const stone of stones) {
    if (stone === 0) {
      output.push(1);
    } else if (`${stone}`.length % 2 === 0) {
      const stoneString = `${stone}`;
      output.push(+stoneString.slice(0, stoneString.length / 2));
      output.push(+stoneString.slice(stoneString.length / 2));
    } else {
      output.push(stone * 2024);
    }
  }
  return output;
}

function stoneBlinksTo(
  stone: number,
  timesToExpand: number,
  stoneTimesMap: Map<string, number> = new Map(),
): number {
  if (timesToExpand === 0) {
    return 1;
  }
  const stoneTimesString = `${stone},${timesToExpand}`;
  if (stoneTimesMap.has(stoneTimesString)) {
    return stoneTimesMap.get(stoneTimesString)!;
  }
  let expanded: number;
  if (stone === 0) {
    expanded = stoneBlinksTo(1, timesToExpand - 1, stoneTimesMap);
  } else if (`${stone}`.length % 2 === 0) {
    const stoneString = `${stone}`;

    expanded =
      stoneBlinksTo(
        +stoneString.slice(0, stoneString.length / 2),
        timesToExpand - 1,
        stoneTimesMap,
      ) +
      stoneBlinksTo(
        +stoneString.slice(stoneString.length / 2),
        timesToExpand - 1,
        stoneTimesMap,
      );
  } else {
    expanded = stoneBlinksTo(stone * 2024, timesToExpand - 1, stoneTimesMap);
  }
  stoneTimesMap.set(stoneTimesString, expanded);

  return expanded;
}

function blinkTimes(stones: number[]): number {
  let times25 = 0;
  for (const stone of stones) {
    let currentStones = [stone];
    for (let i = 0; i < 25; i++) {
      currentStones = blink(currentStones);
    }
    times25 += currentStones.length;
  }
  return times25;
}

const input = util.readInput("11")[0];
const stones = input.split(" ").map((s) => +s);

let part2 = 0;
for (const stone of stones) {
  part2 += stoneBlinksTo(stone, 75);
}

util.writeOutput(blinkTimes(stones), part2);
