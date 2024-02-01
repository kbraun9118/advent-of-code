import * as lib from '../lib';

class Passport {

  private innerStrings: string[];

  constructor(paragraph: string) {
    this.innerStrings = paragraph.split('\n').join(' ').split(' ');
  }

  private getByKey(key: string): string | undefined {
    return this.innerStrings.find(string => string.startsWith(key))?.substring(4)?.trim();
  }

  get birthYear(): string | undefined {
    return this.getByKey('byr');
  }

  get issueYear(): string | undefined {
    return this.getByKey('iyr');
  }

  get expirationYear(): string | undefined {
    return this.getByKey('eyr');
  }

  get height(): string | undefined {
    return this.getByKey('hgt');
  }

  get hairColor(): string | undefined {
    return this.getByKey('hcl');
  }

  get eyeColor(): string | undefined {
    return this.getByKey('ecl');
  }

  get passportId(): string | undefined {
    return this.getByKey('pid');
  }

  get countryId(): string | undefined {
    return this.getByKey('cid');
  }

  isValidPart1(): boolean {
    return this.birthYear !== undefined
      && this.issueYear !== undefined
      && this.expirationYear !== undefined
      && this.height !== undefined
      && this.hairColor !== undefined
      && this.eyeColor !== undefined
      && this.passportId !== undefined;
  }

  isValidPart2(): boolean {
    return this.birthYear !== undefined
      && this.issueYear !== undefined
      && this.expirationYear !== undefined
      && this.height !== undefined
      && this.hairColor !== undefined
      && this.eyeColor !== undefined
      && this.passportId !== undefined
      && +this.birthYear >= 1920 && +this.birthYear <= 2002
      && +this.issueYear >= 2010 && +this.issueYear <= 2020
      && +this.expirationYear >= 2020 && +this.expirationYear <= 2030
      && ((this.height.endsWith('cm') && +this.height.replace('cm', '') >= 150 && +this.height.replace('cm', '') <= 193)
        || (this.height.endsWith('in') && +this.height.replace('in', '') >= 59 && +this.height.replace('in', '') <= 76))
      && !!this.hairColor.match(/^#([0-9]|[a-f]){6}$/)
      && (this.eyeColor.trim() === 'amb'
        || this.eyeColor.trim() === 'blu'
        || this.eyeColor.trim() === 'brn'
        || this.eyeColor.trim() === 'gry'
        || this.eyeColor.trim() === 'grn'
        || this.eyeColor.trim() === 'hzl'
        || this.eyeColor.trim() === 'oth')
      && !!this.passportId.match(/^\d{9}$/);
  }
}

const paragraphs = lib.readParagraphs('04');

const passports = paragraphs.map(paragraph => new Passport(paragraph));

lib.writePart1(passports.map(passport => passport.isValidPart1()).filter(valid => valid).length);

lib.writePart2(passports.map(passport => passport.isValidPart2()).filter(valid => valid).length);
