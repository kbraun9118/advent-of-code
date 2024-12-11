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

function blinkTimes(stones: number[]): [ number, number ] {
  let times25 = 0;
  for (let i = 0; i < 75; i++) {
    console.log(i, stones.length);
    stones = blink(stones);
    if (i==24) {
      times25 = stones.length
    }
  }
  return [times25, stones.length];
}

const input = util.readInput("11")[0];
const stones = input.split(" ").map((s) => +s);

util.writeOutput(...blinkTimes(stones));
