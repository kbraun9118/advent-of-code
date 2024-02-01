import * as lib from '../lib';
import exp from 'constants';

type RuleDictionary = { [key: number]: string; };

const buildRules = (dictionary: RuleDictionary, rule: number, expandedDictionary: { [key: number]: string[] }): string[] => {
  if (expandedDictionary[rule]) {
    return expandedDictionary[rule];
  }
  let returned;
  if (dictionary[rule] === ' \"a\"') {
    returned = ['a'];
  } else if (dictionary[rule] === ' \"b\"') {
    returned = ['b'];
  } else {
    returned = dictionary[rule]
      .split('|')
      .map(ors =>
        ors
          .trim()
          .split(' ')
          .map(nums => +nums)
          .map(nums => buildRules(dictionary, nums, expandedDictionary))
          .reduce((acc, next) => acc
            .map(accRule => next.map(nextRule => accRule + nextRule))
            .reduce((innerAcc, innerNext) => [...innerAcc, ...innerNext], []),
          )
      ).reduce((acc, next) => [...acc, ...next]);
  }
  expandedDictionary[rule] = returned;
  return returned;
};

const lines = lib.readLines('19');

const rules = lines.slice(0, lines.findIndex(value => value === ''));
const messages = lines.slice(lines.findIndex(value => value === '') + 1);

const dictionary: RuleDictionary = {};

rules.forEach(rule => {
  const [ruleNum, rules] = rule.split(':');
  dictionary[+ruleNum] = rules;
});
const expandedDictionary = {}
const expandedRules = buildRules(dictionary, 0, expandedDictionary);
const part1 = messages.filter(message => expandedRules.some(rule => rule === message));
lib.writePart1(part1.length);

// dictionary[8] = ' 42 | 42 8';
// dictionary[11] = ' 42 31 | 42 11 31';
//
// lib.writePart2(messages.filter(message => buildRules(dictionary, 0, {}).some(rule => rule === message)).length);
