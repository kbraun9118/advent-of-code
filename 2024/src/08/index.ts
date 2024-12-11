import util, { Position } from "../util";

function calcAnitnodesPartOne(
  positionOne: Position,
  positionTwo: Position,
): [Position, Position] {
  const differenceX = positionOne.x - positionTwo.x;
  const differenceY = positionOne.y - positionTwo.y;

  return [
    { x: positionOne.x + differenceX, y: positionOne.y + differenceY },
    { x: positionTwo.x - differenceX, y: positionTwo.y - differenceY },
  ];
}

function calcInRange(antinode: Position, mapSize: Position): boolean {
  return (
    antinode.x >= 0 &&
    antinode.y >= 0 &&
    antinode.x < mapSize.x &&
    antinode.y < mapSize.y
  );
}

function calcAnitnodesPartTwo(
  positionOne: Position,
  positionTwo: Position,
  mapSize: Position,
): Position[] {
  const differenceX = positionOne.x - positionTwo.x;
  const differenceY = positionOne.y - positionTwo.y;
  let current = {
    x: positionOne.x + differenceX,
    y: positionOne.y + differenceY,
  };

  const antiNodes: Position[] = [positionTwo, positionOne];
  while (calcInRange(current, mapSize)) {
    antiNodes.push(current);
    current = {
      x: current.x + differenceX,
      y: current.y + differenceY,
    };
  }
  current = {
    x: positionTwo.x - differenceX,
    y: positionTwo.y - differenceY,
  };
  while (calcInRange(current, mapSize)) {
    antiNodes.push(current);
    current = {
      x: current.x - differenceX,
      y: current.y - differenceY,
    };
  }

  return antiNodes;
}

function calcAntinodesForMap(
  antenaMap: Record<string, Position[]>,
  mapSize: Position,
  antiNodeCalculator: (
    positionOne: Position,
    positionTwo: Position,
    mapSize: Position,
  ) => Position[],
): number {
  const positionSet = new Set<string>();
  for (const matchingFreq of Object.values(antenaMap)) {
    for (let i = 0; i < matchingFreq.length - 1; i++) {
      for (let j = i + 1; j < matchingFreq.length; j++) {
        for (const antinode of antiNodeCalculator(
          matchingFreq[i],
          matchingFreq[j],
          mapSize,
        )) {
          if (calcInRange(antinode, mapSize)) {
            positionSet.add(util.positionString(antinode));
          }
        }
      }
    }
  }
  return positionSet.size;
}

const input = util.readInput("08").map((s) => s.split(""));
const antenaMap: Record<string, Position[]> = {};
const mapSize = { x: input[0].length, y: input.length };

for (let y = 0; y < input.length; y++) {
  for (let x = 0; x < input[y].length; x++) {
    if (input[y][x] !== ".") {
      if (antenaMap[input[y][x]]) {
        antenaMap[input[y][x]].push({ x, y });
      } else {
        antenaMap[input[y][x]] = [{ x, y }];
      }
    }
  }
}

const partOne = calcAntinodesForMap(antenaMap, mapSize, calcAnitnodesPartOne);
const partTwo = calcAntinodesForMap(antenaMap, mapSize, calcAnitnodesPartTwo);

util.writeOutput(partOne, partTwo);
