import * as lib from '../lib';

class Password {

  private constructor(
    private rangeStart: number,
    private rangeEnd: number,
    private letter: string,
    private password: string,
  ) {
  }

  static fromString(input: string): Password {
    let split = input.split(':');
    const letter = split[0].substring(split[0].length - 1);
    const range = split[0].substring(0, split[0].length - 1).split('-');
    return new Password(
      +range[0],
      +range[1],
      letter,
      split[1].trim()
    )
  }

  isValidPart1(): boolean {
    const occurrences = this.password.split('').filter(char => char === this.letter).length;
    return occurrences >= this.rangeStart && occurrences <= this.rangeEnd;
  }

  isValidPart2(): boolean {
    const chars = this.password.split('');
    return (chars[this.rangeStart - 1] === this.letter && chars[this.rangeEnd - 1] !== this.letter)
      || (chars[this.rangeStart - 1] !== this.letter && chars[this.rangeEnd - 1] === this.letter);
  }
}

const lines = lib.readLines('02');

let passwords = lines.map(Password.fromString);
console.log('Part 1: ' + passwords.filter(password => password.isValidPart1()).length);
console.log('Part 2: ' + passwords.filter(password => password.isValidPart2()).length);
