import * as lib from '../lib';

class Ticket {
  private row: number;
  private column: number;

  constructor(input: string) {
    const rows = input.substring(0, 7).split('');
    const columns = input.substr(7, input.length).split('');
    let lower = 0;
    let upper = 127;

    while (rows.length > 0) {
      const head = rows.shift();
      const pivot = (Math.floor((upper - lower) / 2)) + lower;
      if (head === 'F') {
        upper = pivot;
      } else {
        lower = pivot;
      }
    }

    this.row = upper;
    upper = 7;
    lower = 0;

    while (columns.length > 0) {
      const head = columns.shift();
      const pivot = (Math.floor((upper - lower) / 2)) + lower;
      if (head === 'L') {
        upper = pivot;
      } else {
        lower = pivot;
      }
    }

    this.column = upper;
  }

  static fromString(input: string): Ticket {
    return new Ticket(input);
  }

  get seatId(): number {
    return this.row * 8 + this.column;
  }
}

const lines = lib.readLines('05');

const ticketIds = lines.map(Ticket.fromString)
  .map(ticket => ticket.seatId)
  .sort((l, r) => l - r);

lib.writePart1(ticketIds[ticketIds.length - 1]);

const missingIds = [];
for (let i = ticketIds[0]; i < ticketIds[ticketIds.length - 1]; i++) {
  if (!ticketIds.find(id => id === i)) {
    missingIds.push(i);
  }
}
lib.writePart2(missingIds[0]);
