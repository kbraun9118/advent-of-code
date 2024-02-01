import * as lib from '../lib';
import Long from 'long';

const lines = lib.readLines('14');

const maskNumber1 = (input: number, mask: string): Long => {
  const mask0 = Long.fromString(mask.split('').map(char => char === '0' ? 0 : 1).join(''), true, 2);
  const mask1 = Long.fromString(mask.split('').map(char => char === '1' ? 1 : 0).join(''), true, 2);
  return Long.fromInt(input, true).or(mask1).and(mask0);
};

let mask = '';
let registers: { [key: string]: Long } = {};

for (const line of lines) {
  if (/^mask\s=\s([X01]*)$/.test(line)) {
    mask = line.replace(/^mask\s=\s([X01]*)$/, '$1');
  } else {
    const [register, value] = line.replace(/mem\[(\d+)]\s=\s(\d+)/, '$1,$2').split(',');
    registers[register] = maskNumber1(+value, mask);
  }
}
lib.writePart1(Object.values(registers).reduce((acc, next) => acc.add(next), Long.fromInt(0, true)).toString());

mask = '';
const registers2: { [register: string]: number } = {};

const padRegister = (register: string) => Long.fromString(register).toString(2).padStart(36, '0');

const maskNumber2 = (address: string, mask: string): string => {
  const maskList = mask.split('');
  return padRegister(address).split('').map((char, i) => {
    if (maskList[i] === '0') {
      return char;
    } else if (maskList[i] === '1') {
      return 1;
    } else {
      return 'X';
    }
  }).join('');
};

const getRegisters = (maskedRegister: string): string[] => {
  let firstX = maskedRegister.indexOf('X');
  if (firstX === -1) {
    return [maskedRegister];
  } else {
    return [
      ...getRegisters(maskedRegister.replace('X', '0')),
      ...getRegisters(maskedRegister.replace('X', '1')),
    ];
  }
};

for (const line of lines) {
  if (/^mask\s=\s([X01]*)$/.test(line)) {
    mask = line.replace(/^mask\s=\s([X01]*)$/, '$1');

  } else {
    const [register, value] = line.replace(/mem\[(\d+)]\s=\s(\d+)/, '$1,$2').split(',');
    for (const decodedRegister of getRegisters(maskNumber2(register, mask))) {
      registers2[decodedRegister] = +value;
    }
  }
}

lib.writePart2(Object.values(registers2).reduce((acc, next) => acc + next));
