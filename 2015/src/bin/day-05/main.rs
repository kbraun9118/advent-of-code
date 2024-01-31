use aoc_2015::lines_for_day;
use std::collections::HashMap;

fn main() {
    let lines = lines_for_day("05");

    let part_one = lines
        .iter()
        .map(|s| part_one_is_nice(s))
        .filter(|p| *p)
        .count();

    let part_two = lines
        .iter()
        .map(|s| part_two_is_nice(s))
        .filter(|p| *p)
        .count();

    println!("Part One: {}", part_one);
    println!("Part Two: {}", part_two);
}

fn part_one_is_nice(input: &str) -> bool {
    let char_map: HashMap<char, i32> = input.chars().fold(HashMap::new(), |mut acc, next| {
        *acc.entry(next).or_default() += 1;
        acc
    });

    let has_double = input
        .chars()
        .zip(input.chars().skip(1))
        .any(|(l, r)| l == r);
    let two_vowels = char_map.get(&'a').unwrap_or(&0)
        + char_map.get(&'e').unwrap_or(&0)
        + char_map.get(&'i').unwrap_or(&0)
        + char_map.get(&'o').unwrap_or(&0)
        + char_map.get(&'u').unwrap_or(&0)
        > 2;
    let no_bad = !input.contains("ab")
        && !input.contains("cd")
        && !input.contains("pq")
        && !input.contains("xy");

    has_double && two_vowels && no_bad
}

fn part_two_is_nice(input: &str) -> bool {
    let repeats_with_middle = input
        .chars()
        .zip(input.chars().skip(1))
        .zip(input.chars().skip(2))
        .any(|((l, _), r)| l == r);

    let pair_repeats = input.chars().zip(input.chars().skip(1)).any(|(l, r)| {
        input
            .replacen(&format!("{}{}", l, r), " ", 1)
            .contains(&format!("{}{}", l, r))
    });

    repeats_with_middle && pair_repeats
}
