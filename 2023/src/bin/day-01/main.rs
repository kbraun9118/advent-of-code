use std::collections::HashMap;

use aoc_2023::{print_part_1, print_part_2, read_input_lines};

fn part_1(lines: &Vec<String>) -> u32 {
    lines
        .iter()
        .map(|line| {
            let left = line.chars().find(|c| c.is_numeric());
            let right = line.chars().rev().find(|c| c.is_numeric());
            format!("{}{}", left.unwrap(), right.unwrap())
                .parse::<u32>()
                .unwrap()
        })
        .sum()
}

fn part_2(lines: &Vec<String>) -> u32 {
    let number_map = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let find_start = |line: &String| {
        for i in 0..line.len() {
            if line.chars().nth(i).unwrap().is_numeric() {
                return line
                    .chars()
                    .nth(i)
                    .unwrap()
                    .to_string()
                    .parse::<u32>()
                    .unwrap();
            }
            for (key, value) in &number_map {
                if line[i..].starts_with(key) {
                    return *value;
                }
            }
        }
        0
    };
    let find_end = |line: &String| {
        for i in (0..line.len()).rev() {
            if line.chars().nth(i).unwrap().is_numeric() {
                return line
                    .chars()
                    .nth(i)
                    .unwrap()
                    .to_string()
                    .parse::<u32>()
                    .unwrap();
            }
            for (key, value) in &number_map {
                if line[i..].starts_with(key) {
                    return *value;
                }
            }
        }
        0
    };
    lines
        .iter()
        .map(|line| {
            let start = find_start(line);
            let end = find_end(line);
            start * 10 + end
        })
        .sum()
}

fn main() {
    let lines = read_input_lines("01");
    print_part_1(part_1(&lines));
    print_part_2(part_2(&lines));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        assert_eq!(part_1(&input), 142);
    }

    #[test]
    fn test_part_2() {
        let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        assert_eq!(part_2(&input), 281);
    }
}
