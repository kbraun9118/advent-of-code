import * as lib from '../lib';

type Ranges = { lower1: number; upper1: number; lower2: number; upper2: number; };

class TicketValidator {
  rules: { [rule: string]: Ranges } = {};

  constructor(
    ruleStrings: string[],
  ) {
    ruleStrings.forEach(rule => {
      const [ruleName, bounds] = rule.split(':');
      const [lower1, upper1, lower2, upper2] = bounds
        .replace(/(\d+)-(\d+)\sor\s(\d+)-(\d+)/, '$1,$2,$3,$4')
        .split(',')
        .map(val => +val);
      this.rules[ruleName] = { lower1, upper1, lower2, upper2 };
    });
  }

  findInvalidField(ticket: number[]): number | undefined {
    for (const field of ticket) {
      if (!Object.values(this.rules)
        .find(({
                 lower1,
                 upper1,
                 lower2,
                 upper2,
               }) => (field >= lower1 && field <= upper1) || (field >= lower2 && field <= upper2))) {
        return field;
      }
    }
    return undefined;
  }

  getRules(): string[] {
    return Object.keys(this.rules);
  }

  validForRule(ticketField: number, rule: string): boolean {
    return (ticketField >= this.rules[rule].lower1 && ticketField <= this.rules[rule].upper1)
      || (ticketField >= this.rules[rule].lower2 && ticketField <= this.rules[rule].upper2);
  }
}

const lines = lib.readLines('16');

const rules = lines.slice(0, lines.indexOf(''));
const myTicket = lines[lines.indexOf('your ticket:') + 1].split(',').map(val => +val);
const nearbyTickets = lines.slice(lines.indexOf('nearby tickets:') + 1, lines.length).map(ticket => ticket.split(',').map(val => +val));

const validator = new TicketValidator(rules);

const part1 = (validator: TicketValidator, nearbyTickets: number[][]): number => {
  return nearbyTickets.map(ticket => validator.findInvalidField(ticket))
    .filter(invalid => invalid !== undefined)
    .reduce((acc, next) => acc!! + next!!)!!;
};

const part2 = (validator: TicketValidator, nearbyTickets: number[][], myTicket: number[]): number => {
  const validTickets = nearbyTickets.filter(ticket => validator.findInvalidField(ticket) === undefined);
  let validRulesForField = myTicket.map(() => validator.getRules());
  for (const validTicket of validTickets) {
    for (const [fieldIndex, ticketField] of validTicket.entries()) {
      validRulesForField[fieldIndex] = validRulesForField[fieldIndex].filter(rule => validator.validForRule(ticketField, rule));
    }
  }
  const columns = validRulesForField.map(() => '');
  while (validRulesForField.reduce((acc, next) => acc + next.length, 0)) {
    validRulesForField.forEach((rulesForField, i) => {
      if (rulesForField.length === 1) {
        columns[i] = rulesForField[0];
        for (let j = 0; j < validRulesForField.length; j++) {
          validRulesForField[j] = validRulesForField[j].filter(name => name !== columns[i]);
        }
      }
    });
  }
  return columns.map((column, i) => ({ column, i }))
    .filter(({ column }) => column.startsWith('departure'))
    .reduce<number>((acc, { i }) => acc * myTicket[i], 1);
};

lib.writePart1(part1(validator, nearbyTickets));
lib.writePart2(part2(validator, nearbyTickets, myTicket));
