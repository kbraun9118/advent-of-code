import exp from 'constants';

const lib = require('./index');

describe('lib.Definition', () => {
  it('should determine if inputs match', () => {
    let definition = new lib.Definition('0 1 | 2 3');
    expect(definition.matches(['0', '1'])).toBe(true);
    expect(definition.matches(['2', '3'])).toBe(true);
    expect(definition.matches(['4', '5'])).toBe(false);
    expect(definition.matches(['0'])).toBe(false);
    expect(definition.matches(['1'])).toBe(false);

    expect(new lib.Definition('"a"').matches(['a'])).toBe(true);
  });
});

describe('CYK', () => {
  it('should do something', () => {
    let grammar = new lib.Grammar([
      '42: 9 14 | 10 1',
      '9: 14 27 | 1 26',
      '10: 23 14 | 28 1',
      '1: "a"',
      '11: 42 31',
      '5: 1 14 | 15 1',
      '19: 14 1 | 14 14',
      '12: 24 14 | 19 1',
      '16: 15 1 | 14 14',
      '31: 14 17 | 1 13',
      '6: 14 14 | 1 14',
      '2: 1 24 | 14 4',
      '0: 8 11',
      '13: 14 3 | 1 12',
      '15: 1 | 14',
      '17: 14 2 | 1 7',
      '23: 25 1 | 22 14',
      '28: 16 1',
      '4: 1 1',
      '20: 14 14 | 1 15',
      '3: 5 14 | 16 1',
      '27: 1 6 | 14 18',
      '14: "b"',
      '21: 14 1 | 1 14',
      '25: 1 1 | 1 14',
      '22: 14 14',
      '8: 42',
      '26: 14 22 | 1 20',
      '18: 15 15',
      '7: 14 5 | 1 21',
      '24: 14 1',
    ]);

    expect(grammar.matchingRules(['1', '14'])).toEqual([5, 6, 21, 25]);
    expect(grammar.matchingRules(['a'])).toEqual([1]);
    expect(grammar.matchingRules(['b'])).toEqual([14]);
    expect(grammar.matchingRules(['b'])).toEqual([14]);
    expect(grammar.matchingRules(['42'])).toEqual([8]);
  });
});

test('cartesian', () => {
  expect(lib.cartesian(['1', '2'], ['3', '4'])).toEqual([['1', '3'], ['1', '4'], ['2', '3'], ['2', '4']]);
})

test('cyk', () => {
  let grammar = new lib.Grammar([
    '42: 9 14 | 10 1',
    '9: 14 27 | 1 26',
    '10: 23 14 | 28 1',
    '1: "a"',
    '11: 42 31 | 42 99',
    '5: 1 14 | 15 1',
    '19: 14 1 | 14 14',
    '12: 24 14 | 19 1',
    '16: 15 1 | 14 14',
    '31: 14 17 | 1 13',
    '6: 14 14 | 1 14',
    '2: 1 24 | 14 4',
    '0: 8 11',
    '13: 14 3 | 1 12',
    '15: 1 | 14',
    '17: 14 2 | 1 7',
    '23: 25 1 | 22 14',
    '28: 16 1',
    '4: 1 1',
    '20: 14 14 | 1 15',
    '3: 5 14 | 16 1',
    '27: 1 6 | 14 18',
    '14: "b"',
    '21: 14 1 | 1 14',
    '25: 1 1 | 1 14',
    '22: 14 14',
    '8: 42 | 42 8',
    '26: 14 22 | 1 20',
    '18: 15 15',
    '7: 14 5 | 1 21',
    '24: 14 1',
    '99: 11 31',

    // /**
    //  * S = 0
    //  * A = 1
    //  * B = 2
    //  * C = 3
    //  * D = 4
    //  * F = 5
    //  * E = 6
    //  */
    // '0: 1 2',
    // '1: 3 4 | 3 5',
    // '2: 6 6 | 6 2',
    // '3: "a"',
    // '4: "b"',
    // '6: "c"',
    // '5: 1 4',
  ]);

  expect(lib.cyk(grammar, 'ababaaaaabbbaba')).toEqual(true);
  // expect(lib.cyk(grammar, 'aaabbbccc')).toEqual(true);
})
