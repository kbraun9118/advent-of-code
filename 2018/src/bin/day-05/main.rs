#![allow(unused)]

use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::io::BufRead;
use std::str::Chars;

fn main() {
    let reader = BufReader::new(
        File::open(Path::new("../input/2018/05/input.txt")).unwrap());
    let lines = reader.lines()
        .map(|result| result.unwrap())
        .find(|_| true).unwrap();

//    let mut lines = "dabAcCaCBAcCcaDA".to_string();
    let mut part2 = lines.as_str().to_string();
    let mut part1: String = lines.as_str().to_string();

    part1 = reduce(part1);


    let letters = "abcdefghijklmnopqrstuvwxyz".to_string();

    let answer2 = letters.chars()
        .map(|letter| {
            let removed = part2.chars()
                .filter(|poly| letter != poly.to_ascii_lowercase())
                .collect::<String>();
            reduce(removed).len()
        }
        ).min_by(|left, right| left.cmp(right));


    println!("{}", part1.len());
    if let Some(size) = answer2 {
        println!("{:?}", size);
    }
}

fn reduce(string: String) -> String {
    let mut part1 = string;
    while contains_polymer(&part1) {
        let mut left_chars = part1.chars().peekable();
        let mut right_chars = part1.chars().peekable();
        right_chars.next();
        let mut polymer = "".to_string();

        loop {
            match (left_chars.peek(), right_chars.peek()) {
                (Some(left), Some(right)) => {
                    if test_polymer(*left, *right) {
                        left_chars.next();
                        left_chars.next();
                        right_chars.next();
                        right_chars.next();
                    } else {
                        polymer.push(*left);
                        left_chars.next();
                        right_chars.next();
                    }
                }
                (Some(left), None) => {
                    polymer.push(*left);
                    left_chars.next();
                }
                (None, _) => {
                    break;
                }
            }
        }
        part1 = polymer;
    }
    part1
}

fn contains_polymer(string: &str) -> bool {
    let mut peekable = string.chars().peekable();
    while let Some(value) = peekable.next() {
        if let Some(inner) = peekable.peek() {
            if test_polymer(value, *inner) { return true; }
        }
    }
    false
}

fn test_polymer(char_l: char, char_r: char) -> bool {
    char_r != char_l && (char_r == char_l.to_ascii_uppercase() || char_r == char_l.to_ascii_lowercase())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_test_polymer() {
        assert!(test_polymer('a', 'A'));
        assert!(test_polymer('A', 'a'));
        assert_eq!(test_polymer('A', 'A'), false);
        assert_eq!(test_polymer('a', 'a'), false);
    }

    #[test]
    fn test_contains_polymer() {
        let string = "asdfqwer".to_string();
        let string2 = "aAfqwerqwer".to_string();
        assert_eq!(contains_polymer(&string), false);
        assert!(contains_polymer(&string2));
    }
}
