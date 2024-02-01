import { readFileSync } from 'fs';

export const readLines = (day: string): string[] => {
  const file = readFileSync(`../input/2020/${day}/input.txt`, 'utf8');
  return file.substring(0, file.length - 1).split('\r').join('').split('\n');
};

export const readParagraphs = (day: string): string[] => {
  const file = readFileSync(`../input/2020/${day}/input.txt`, 'utf8');
  return file.substring(0, file.length - 1).split('\r').join('').split('\n\n');
};

export const writePart1 = (answer: any) => {
  console.log('Part 1: ' + answer);
};

export const writePart2 = (answer: any) => {
  console.log('Part 2: ' + answer);
};

export class Grammar {
  grammar: { [rule: number]: Definition; };

  constructor(input: string[]) {
    this.grammar = input.reduce<{ [rule: number]: Definition; }>((acc, next) => {
      const [rule, def] = next.split(':');
      acc[+(rule.trim())] = new Definition(def);
      return acc;
    }, {});
  }

  matchingRules(input: string[]): number[] {
    let map = Object.entries(this.grammar)
      .filter(([, definition]) => definition.matches(input))
      .map(([rule]) => +rule);
    let previousLength;
    do {
      previousLength = map.length;
      map = [...map.map(rule => this.matchingRules([rule.toString()]))
        .reduce((acc, next) => {
          next.forEach(inner => acc.add(inner));
          return acc;
        }, new Set(map))]
    } while (previousLength !== map.length);

    return map;
  }

  print() {
    console.log(JSON.stringify(this.grammar));
  }
}

export class Definition {
  readonly isTerminal: boolean;
  readonly defines: string[][];

  constructor(rule: string) {
    this.defines = rule.split('|').map(inner => inner.trim().split('').filter(str => str !== '"').join('').split(' ').map(num => num));
    this.isTerminal = this.defines.every(inner => inner.every(value => isNaN(+value)));
  }

  matches(input: string[]): boolean {
    const joinedInput = input.join(' ');
    if (this.defines.length === 2) {
      if (this.defines[1].join(' ') === joinedInput) {
        return true;
      }
    }
    return this.defines[0].join(' ') === joinedInput;
  }

  print() {
    console.log(JSON.stringify(this.defines));
  }
}

export const cartesian = (left: string[], right: string[]): string[][] => {
  let reduce = left.map(innerL => right.map(innerR => [innerL, innerR]))
    .reduce<string[][]>((acc, next) => [...acc, ...next], []);
  console.log(`Cartesian of (left=${left}) (right=${right}) is ${JSON.stringify(reduce)}`)
  return reduce;
};

export const cyk = (grammar: Grammar, input: string): boolean => {
  let inputArr = input.split('');
  const matrix: (number[] | undefined)[][] = [];
  // initialize NxN matrix
  for (let i = 0; i < input.length; i++) {
    matrix[i] = [];
    for (let j = 0; j < input.length; j++) {
      matrix[i][j] = undefined;
    }
  }

  inputArr.forEach((char, i) => matrix[0][i] = grammar.matchingRules([char]));

  for (let i = 1; i < matrix.length; i++) {
    for (let j = 0; j < matrix.length - i; j++) {
      for (let k = 0; k < i; k++) {

        console.log(`For (${j}, ${i}) Looking at (${j}, ${k}): ${matrix[k][j]} and (${k + 1 + j}, ${i- 1 - k}): ${matrix[i- 1 - k][k + 1 + j]}`);
        if (matrix[k][j] && matrix[i- 1 - k][k + 1 + j]) {
          const patternsSet = cartesian(matrix[k][j]!!.map(chars => chars.toString()), matrix[i - 1 - k][k + 1 + j]!!.map(chars => chars.toString()))
            .map(pattern => grammar.matchingRules(pattern))
            .reduce<Set<number>>((acc, next) => {
              next.forEach(num => acc.add(num));
              return acc;
            }, new Set());
          const patterns = [...patternsSet];
          console.log('Setting to ', patterns)
          if (patterns.length > 0) {
            matrix[i][j] = patterns;
          }
        }
      }
    }
  }

  console.log(JSON.stringify(matrix));

  return matrix[matrix.length - 1][0]?.some(val => val === 0) || false;
};
