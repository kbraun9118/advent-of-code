import util from "../util";

type Location = {
  x: number;
  y: number;
};

function findXs(search: string[]): Location[] {
  const xs: Location[] = [];
  for (const [y, line] of search.entries()) {
    for (const [x, char] of line.split("").entries()) {
      if (char === "X") {
        xs.push({ x, y });
      }
    }
  }
  return xs;
}

function countXmasForX(
  search: string[],
  word: string,
  location: Location,
): number {
  let wordCount = 0;
  if (location.x + word.length <= search[0].length) {
    const searchWord = search[location.y].substring(
      location.x,
      location.x + word.length,
    );
    if (searchWord === word) {
      wordCount++;
    }
  }
  if (location.x - word.length + 1 >= 0) {
    const searchWord = search[location.y]
      .substring(location.x - word.length + 1, location.x + 1)
      .split("")
      .reverse()
      .join("");

    if (searchWord === word) {
      wordCount++;
    }
  }
  if (location.y + word.length <= search.length) {
    let searchWord = "";
    for (let i = 0; i < word.length; i++) {
      searchWord += search[location.y + i][location.x];
    }
    if (searchWord === word) {
      wordCount++;
    }
  }
  if (location.y - word.length + 1 >= 0) {
    let searchWord = "";
    for (let i = 0; i < word.length; i++) {
      searchWord += search[location.y - i][location.x];
    }
    if (searchWord === word) {
      wordCount++;
    }
  }
  if (
    location.x + word.length <= search[0].length &&
    location.y + word.length <= search.length
  ) {
    let searchWord = "";
    for (let i = 0; i < word.length; i++) {
      searchWord += search[location.y + i][location.x + i];
    }
    if (searchWord === word) {
      wordCount++;
    }
  }
  if (
    location.x - word.length + 1 >= 0 &&
    location.y + word.length <= search.length
  ) {
    let searchWord = "";
    for (let i = 0; i < word.length; i++) {
      searchWord += search[location.y + i][location.x - i];
    }
    if (searchWord === word) {
      wordCount++;
    }
  }
  if (location.x - word.length + 1 >= 0 && location.y - word.length + 1 >= 0) {
    let searchWord = "";
    for (let i = 0; i < word.length; i++) {
      searchWord += search[location.y - i][location.x - i];
    }
    if (searchWord === word) {
      wordCount++;
    }
  }
  if (
    location.x + word.length <= search[0].length &&
    location.y - word.length + 1 >= 0
  ) {
    let searchWord = "";
    for (let i = 0; i < word.length; i++) {
      searchWord += search[location.y - i][location.x + i];
    }
    if (searchWord === word) {
      wordCount++;
    }
  }
  return wordCount;
}

function findCrossMas(search: string[]): number {
  let crossMas = 0;
  for (let y = 0; y + 2 < search.length; y++) {
    for (let x = 0; x + 2 < search[y].length; x++) {
      const downLeft =
        search[y][x] + search[y + 1][x + 1] + search[y + 2][x + 2];
      const upRight =
        search[y + 2][x] + search[y + 1][x + 1] + search[y][x + 2];
      if (
        (downLeft === "MAS" || downLeft === "SAM") &&
        (upRight === "MAS" || upRight === "SAM")
      ) {
        crossMas++;
      }
    }
  }

  return crossMas;
}

const input = util.readInput("04");
const xs = findXs(input);

const part1 = xs
  .map((xLoc) => countXmasForX(input, "XMAS", xLoc))
  .reduce((acc, curr) => acc + curr);

const part2 = findCrossMas(input);

util.writeOutput(part1, part2);
