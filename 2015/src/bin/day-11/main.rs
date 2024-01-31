use std::collections::VecDeque;

use aoc_2015::lines_for_day;
use regex::Regex;

fn main() {
    let input = lines_for_day("11").pop().expect("No Input");
    let part_one_answer = next_valid_password(&input);
    println!("Part One: {}", part_one_answer);
    let part_two_answer = increment_string(&part_one_answer);
    println!("Part Two: {}", next_valid_password(&part_two_answer));
}

fn next_valid_password(string: &str) -> String {
    let mut current = string.to_string();
    while !legal_password(&current) {
        current = increment_string(&current);
    }
    current
}

fn legal_password(string: &str) -> bool {
    let output = string
        .as_bytes()
        .windows(3)
        .any(|w| w[0] + 1 == w[1] && w[0] + 2 == w[2]);

    let output = output
        && string.as_bytes().windows(2).any(|w| {
            w[0] == w[1]
                && string
                    .replace(&format!("{}{}", w[0] as char, w[1] as char), " ")
                    .as_bytes()
                    .windows(2)
                    .any(|ww| ww[0] == ww[1])
        });

    output && !string.contains(&Regex::new("(i|o|l)").expect("Invalid Regex"))
}

fn increment_string(string: &str) -> String {
    let mut chars = string.chars().collect::<VecDeque<_>>();
    let mut output = VecDeque::new();
    while let Some(i) = chars.pop_back() {
        let next = increment_char(i);
        output.push_front(next);
        if next == 'a' && chars.is_empty() {
            output.push_front('a');
        } else if next != 'a' {
            break;
        }
    }

    chars.append(&mut output);
    chars.into_iter().collect()
}

fn increment_char(c: char) -> char {
    let output = (c as u8 + 1) as char;
    if output == '{' {
        'a'
    } else {
        output
    }
}
