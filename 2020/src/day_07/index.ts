import * as lib from '../lib';

class Bag {

  private constructor(
    public color: string,
    public contents: [number, string][],
  ) {
  }

  static fromString(input: string): Bag {
    const color = input.substring(0, input.indexOf(' bags contain'));
    if (input.substring(input.indexOf('contain') + 8, input.length) === 'no other bags.') {
      return new Bag(color, []);
    }
    const contents: [number, string][] = input.substring(input.indexOf('contain') + 8, input.length).split(',')
      .map(content => {
        const values = content.replace('.', '').trim().split(' ');
        return [+values[0], `${values[1]} ${values[2]}`];
      });

    return new Bag(color, contents);
  }

  static goldBag(): Bag {
    return new Bag('shiny gold', []);
  }

  canContain(bag: Bag): boolean {
    return !!this.contents.find(content => content[1] === bag.color);
  }

  isGold(): boolean {
    return this.color === 'shiny gold';
  }
}

const lines = lib.readLines('07');

const bags = lines.map(Bag.fromString);
let bagsToSearch = [...bags];

const goldBag = bags.find(bag => bag.isGold()) || Bag.goldBag();

const containsGold: Bag[] = [];
let previous = -1;

while (containsGold.length !== previous) {
  previous = containsGold.length;
  const bagsAdded = [];
  for (const bag of bagsToSearch) {
    if (bag.canContain(goldBag) || !!containsGold.find(contain => bag.canContain(contain))) {
      bagsAdded.push(bag);
      containsGold.push(bag);
    }
    bagsAdded.forEach(bag => {
      bagsToSearch = bagsToSearch.filter(search => search !== bag);
    });
  }
}

lib.writePart1(containsGold.length);

class BagTreeNode {

  constructor(
    private bag: Bag,
    private count: number,
    private leaves: BagTreeNode[],
  ) {
  }

  amountContained(): number {
    if (this.leaves.length === 0) {
      return this.count;
    } else {
      return this.count + this.count * this.leaves.reduce((acc, leaf) => acc + leaf.amountContained(), 0);
    }
  }

  static fromBag(bags: Bag[], bag: Bag, count: number): BagTreeNode {
    if (bag.contents.length === 0) {
      return new BagTreeNode(bag, count, []);
    } else {
      const leaves = bag.contents.map(([count, bagString]) => {
        const bag = bags.find(value => value.color === bagString);
        if (bag) {
          return this.fromBag(bags, bag, count);
        } else  {
          throw new Error('Bag Not found');
        }
      });
      return new BagTreeNode(bag, count, leaves);
    }
  }
}

let goldBagTree = BagTreeNode.fromBag(bags, goldBag, 1);
lib.writePart2(goldBagTree.amountContained() - 1);
