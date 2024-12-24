import util from "../util";

class Computer {
  instructionPointer = 0;
  output: number[] = [];

  constructor(
    public registerA: number,
    public registerB: number,
    public registerC: number,
    public program: number[],
  ) {}

  static fromInput(lines: string[]): Computer {
    const registerA = +lines[0].substring(12);
    const registerB = +lines[1].substring(12);
    const registerC = +lines[2].substring(12);
    const program = lines[4]
      .substring(9)
      .split(",")
      .map((n) => +n);

    return new Computer(registerA, registerB, registerC, program);
  }

  print(): void {
    console.log("Registers ");
    console.log(`A: ${this.registerA}`);
    console.log(`B: ${this.registerB}`);
    console.log(`C: ${this.registerC}`);
    console.log("Program: ", this.program);
  }

  reset(): void {
    this.registerA = 0;
    this.registerB = 0;
    this.registerC = 0;
    this.instructionPointer = 0;
    this.output = [];
  }

  combo(operand: number): number {
    switch (operand) {
      case 4:
        return this.registerA;
      case 5:
        return this.registerB;
      case 6:
        return this.registerC;
      case 7:
        throw new Error("invalid operand");
      default:
        return operand;
    }
  }

  runInstruction(): void {
    switch (this.program[this.instructionPointer]) {
      case 0:
        this.registerA = Math.floor(
          this.registerA /
            Math.pow(2, this.combo(this.program[this.instructionPointer + 1])),
        );
        this.instructionPointer += 2;
        break;
      case 1:
        this.registerB =
          this.registerB ^ this.program[this.instructionPointer + 1];
        this.instructionPointer += 2;
        break;
      case 2:
        this.registerB =
          this.combo(this.program[this.instructionPointer + 1]) % 8;
        this.instructionPointer += 2;
        break;
      case 3:
        if (this.registerA !== 0) {
          this.instructionPointer = this.program[this.instructionPointer + 1];
        } else {
          this.instructionPointer += 2;
        }
        break;
      case 4:
        this.registerB = this.registerB ^ this.registerC;
        this.instructionPointer += 2;
        break;
      case 5:
        this.output.push(
          this.combo(this.program[this.instructionPointer + 1]) % 8,
        );
        this.instructionPointer += 2;
        break;
      case 6:
        this.registerB = Math.floor(
          this.registerA /
            Math.pow(2, this.combo(this.program[this.instructionPointer + 1])),
        );
        this.instructionPointer += 2;
        break;
      case 7:
        this.registerC = Math.floor(
          this.registerA /
            Math.pow(2, this.combo(this.program[this.instructionPointer + 1])),
        );
        this.instructionPointer += 2;
        break;
    }
  }

  runProgram(): number[] {
    while (
      this.instructionPointer >= 0 &&
      this.instructionPointer < this.program.length
    ) {
      this.runInstruction();
    }
    return this.output;
  }

  findCopy(): number {
    let i = 1;
    this.registerA = i;
    let output = this.runProgram();
    while (output.join(",") !== this.program.join(",")) {
      if (i % 10_000 === 0) {
        console.log("here, ", i);
      }
      this.reset();
      i++;
      this.registerA = i;
      output = this.runProgram();
    }
    return i;
  }
}

const input = util.readInput("17");

const computer = Computer.fromInput(input);

const output = computer.runProgram();

// // If register C contains 9, the program 2,6 would set register B to 1.
// const ex1 = new Computer(0, 0, 9, [2, 6]);
// const ex1Out = ex1.runProgram();
// ex1.print();
// console.log(ex1Out);
// // If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
// const ex2 = new Computer(10, 0, 0, [5, 0, 5, 1, 5, 4]);
// const ex2Out = ex2.runProgram();
// ex2.print();
// console.log(ex2Out);
// // If register A contains 2024, the program 0,1,5,4,3,0 would output 4,2,5,6,7,7,7,7,3,1,0 and leave 0 in register A.
// const ex3 = new Computer(2024, 0, 0, [0, 1, 5, 4, 3, 0]);
// const ex3Out = ex3.runProgram();
// ex3.print();
// console.log(ex3Out);
// // If register B contains 29, the program 1,7 would set register B to 26.
// const ex4 = new Computer(0, 29, 0, [1, 7]);
// const ex4Out = ex4.runProgram();
// ex4.print();
// console.log(ex4Out);
// // If register B contains 2024 and register C contains 43690, the program 4,0 would set register B to 44354
// const ex5 = new Computer(0, 2024, 43690, [4, 0]);
// const ex5Out = ex5.runProgram();
// ex5.print();
// console.log(ex5Out);

util.writeOutput(output.join(","), computer.findCopy());
