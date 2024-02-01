import * as lib from '../lib';

const lines = lib.readLines('13');

const timestamp = +lines[0];
const buses = lines[1].split(',').filter(bus => !isNaN(+bus)).map(bus => +bus);

const timeWaiting = buses.map(bus => ({ id: bus, waiting: bus - (timestamp % bus) }));

const leastWaiting = timeWaiting.reduce(
  (acc, next) => acc.waiting < next.waiting ? acc : next,
);

lib.writePart1(leastWaiting.id * leastWaiting.waiting);

const tValues = lines[1]
  .split(',')
  .map((id, t) => ({ id, t }))
  .filter(({ id }) => !isNaN(+id))
  .map(({ id, t }) => ({
    id: +id,
    t,
  }));

let step = 1;
let i = 1;

for (const { id, t } of tValues) {
  while ((i + t) % id !== 0) {
    i += step
  }
  step *= id;
}

lib.writePart2(i);
