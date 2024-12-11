import util from "../util";

function buildDisk(diskMap: string): string[] {
  let blockId = 0;
  const disk: string[] = [];
  for (let i = 0; i < diskMap.length; i += 2) {
    for (let j = 0; j < +diskMap[i]; j++) {
      disk.push("" + blockId);
    }
    blockId++;
    for (let j = 0; j < +diskMap[i + 1]; j++) {
      disk.push(".");
    }
  }
  return disk;
}

function compactDisk(disk: string[]): string[] {
  const compacted = [...disk];
  let front = 0;
  let back = disk.length - 1;

  while (front < back) {
    if (compacted[front] !== ".") {
      front++;
    } else if (compacted[back] === ".") {
      back--;
    } else {
      compacted[front] = compacted[back];
      compacted[back] = ".";
      front++;
      back--;
    }
  }
  const endIdx = compacted.indexOf(".");
  return compacted.slice(0, endIdx);
}

// need to do this in reverse
function defragDisk(disk: string[]): string[] {
  const defragged = [...disk];
  const maxId = Math.max(
    ...defragged.map((id) => +id).filter((id) => !isNaN(id)),
  );

  outer: for (let currentId = maxId; currentId >= 0; currentId--) {
    const idString = currentId.toString();
    const startIdx = defragged.indexOf(idString);
    let idWindow = 0;
    for (
      let i = 0;
      i < defragged.length && defragged[startIdx + i] === idString;
      i++
    ) {
      idWindow++;
    }
    for (let i = 0; i < startIdx; i++) {
      let currentWindow = 0;
      if (defragged[i] === ".") {
        for (let j = 0; defragged[i + j] === "."; j++) {
          currentWindow++;
        }
        if (currentWindow >= idWindow) {
          for (let j = 0; j < idWindow; j++) {
            defragged[i + j] = idString;
            defragged[startIdx + j] = ".";
          }
          continue outer;
        }
        i += currentWindow;
      }
    }
  }

  return defragged;
}

function calcChecksum(compactedDisk: string[]): number {
  return compactedDisk.reduce((acc, curr, idx) => {
    if (isNaN(+curr)) {
      return acc;
    }
    return acc + +curr * idx;
  }, 0);
}

const input = util.readInput("09")[0];

const disk = buildDisk(input);
const compacted = compactDisk(disk);
const checksum = calcChecksum(compacted);

const defragged = defragDisk(disk);
const defraggedChecksum = calcChecksum(defragged);

util.writeOutput(checksum, defraggedChecksum);
