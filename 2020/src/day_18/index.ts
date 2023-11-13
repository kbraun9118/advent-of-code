import * as lib from '../lib';

const shuntingYard = (input: string, precedence: {[key: string]: number}): string[] => {
  const tokens = input.split('').filter(token => token !== ' ');
  const output = [];
  const operators = [];

  while (tokens.length > 0) {
    const next = tokens.shift();
    if (next) {
      if (/\d+/.test(next)) {
        output.push(next);
      } else if (next === '+' || next === '*') {
        while ((operators[operators.length - 1] === '*' || operators[operators.length - 1] === '+')  && precedence[operators[operators.length -1]] >= precedence[next]) {
          output.push(operators.pop()!!);
        }
        operators.push(next);
      } else if (next === '(') {
        operators.push(next);
      } else {
        while (operators[operators.length - 1] !== '(') {
          output.push(operators.pop()!!);
        }
        operators.pop();
      }
    }
  }
  return [...output, ...operators.reverse()];
};

const solveShunting = (input: string[]): number => {
  let tokens = [...input];
  while (tokens.length !== 1) {
    const next: string[] = [];
    for (let i = 0; i < tokens.length; i++) {
      if (/\d+/.test(tokens[i]) && /\d+/.test(tokens[i + 1]) && (tokens[i + 2] === '+' || tokens[i + 2] === '*')) {
        if (tokens[i + 2] === '+') {
          next.push((+tokens[i] + +tokens[i + 1]).toString());
          i += 2;
        } else {
          next.push((+tokens[i] * +tokens[i + 1]).toString());
          i += 2;
        }
      } else {
        next.push(tokens[i]);
      }
    }
    tokens = next;
  }
  return +tokens[0];
};

const lines = lib.readLines(__dirname + '/input.txt');
lib.writePart1(lines.map(line => solveShunting(shuntingYard(line, {'*': 0, '+': 0}))).reduce((acc, next) => {
  return acc + next;
}, 0));
lib.writePart2(lines.map(line => solveShunting(shuntingYard(line, {'*': 0, '+': 1}))).reduce((acc, next) => {
  return acc + next;
}, 0));
