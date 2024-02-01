import * as lib from '../lib';

enum Operation {
  NoOperation = 'nop',
  Accumulator = 'acc',
  Jump = 'jmp',
}

class Instruction {
  constructor(
    public readonly operation: Operation,
    public readonly value: number,
  ) {
  }

  static fromString(input: string): Instruction {
    const split = input.split(' ');
    return new Instruction(split[0] as Operation, +split[1]);
  }

  flip(): Instruction {
    switch (this.operation) {
      case Operation.NoOperation:
        return new Instruction(Operation.Jump, this.value);
      case Operation.Jump:
        return new Instruction(Operation.NoOperation, this.value);
      default:
        return this;
    }
  }
}

const lines = lib.readLines('08');

const instructions = lines.map(Instruction.fromString);

let accumulator = 0;
let instructionPointer = 0;
const instructionsRan = new Set<number>();

while (!instructionsRan.has(instructionPointer)) {
  instructionsRan.add(instructionPointer);
  switch (instructions[instructionPointer].operation) {
    case Operation.Accumulator:
      accumulator += instructions[instructionPointer].value;
      instructionPointer++;
      break;
    case Operation.Jump:
      instructionPointer += instructions[instructionPointer].value;
      break;
    case Operation.NoOperation:
      instructionPointer++;
      break;
    default:
      throw new Error('unexpected operation');
  }
}

lib.writePart1(accumulator);

const movementInstructions = [...instructionsRan]
  .filter(instruction => instructions[instruction].operation === Operation.NoOperation
    || instructions[instruction].operation === Operation.Jump);

for (const instruction of movementInstructions) {
  const instructionsThisRun = [...instructions]
  instructionsThisRun[instruction] = instructionsThisRun[instruction].flip();
  accumulator = 0;
  instructionPointer = 0;
  instructionsRan.clear();
  while (!instructionsRan.has(instructionPointer) && instructionPointer < instructionsThisRun.length) {
    instructionsRan.add(instructionPointer);
    switch (instructionsThisRun[instructionPointer].operation) {
      case Operation.Accumulator:
        accumulator += instructionsThisRun[instructionPointer].value;
        instructionPointer++;
        break;
      case Operation.Jump:
        instructionPointer += instructionsThisRun[instructionPointer].value;
        break;
      case Operation.NoOperation:
        instructionPointer++;
        break;
      default:
        throw new Error('unexpected operation');
    }
  }
  if (instructionPointer >= instructionsThisRun.length) {
    break;
  }
}

lib.writePart2(accumulator);
