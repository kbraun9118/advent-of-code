import * as lib from '../lib';

const paragraphs = lib.readParagraphs('06');

lib.writePart1(
  paragraphs
    .map(para => para.split('\n').reduce((set, answers) => {
      answers.split('')
        .forEach(answer => set.add(answer));
      return set;
    }, new Set<string>()))
    .reduce((acc, answers) => acc + answers.size, 0),
);

lib.writePart2(paragraphs.map(para => para.split('\n').map(answer => answer.split('')))
  .map(answers => {
    let yeses = answers.shift();
    while (answers.length > 0) {
      const next = answers.shift();
      yeses = yeses?.filter(value => next?.includes(value));
    }
    return yeses;
  })
  .map(answers => answers?.length)
  .reduce((acc, next) => (acc || 0) + (next || 0) ,0)
)
