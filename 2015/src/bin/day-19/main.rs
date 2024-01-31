#![feature(exclusive_range_pattern)]
use std::collections::HashSet;

use aoc_2015::lines_for_day;
use regex::Regex;

fn main() {
    let lines = lines_for_day("19");
    let (replacements, molecule) = lines.split_at(lines.len() - 2);
    let molecule = molecule[1].as_str().to_string();

    let mut replacements = replacements
        .into_iter()
        .map(Replacement::from)
        .collect::<Vec<_>>();

    replacements.sort_by_key(|r| r.value.len());
    replacements.reverse();

    let expanded = part_1(&replacements, molecule.clone());

    println!("Part 1: {}", expanded.len());
    println!("Part 2: {}", part_2(&replacements, molecule));
}

fn part_1(replacements: &Vec<Replacement>, molecule: String) -> HashSet<String> {
    let regex = Regex::new(r"[A-Z|e][a-d|f-z]*").unwrap();
    let molecules = regex
        .captures_iter(&molecule)
        .map(|c| c.get(0).unwrap().as_str().to_string())
        .collect::<Vec<_>>();

    molecules
        .iter()
        .enumerate()
        .flat_map(|(i, m)| {
            let mut items = vec![];
            for r in replacements.iter().filter(|r| r.key == *m) {
                let mut new_molecule = molecules.clone();
                new_molecule[i] = r.value.clone();
                items.push(new_molecule.join(""))
            }
            items
        })
        .collect::<HashSet<_>>()
}

fn part_2_conversion(replacements: &Vec<Replacement>, molecule: String) -> String {
    replacements
        .into_iter()
        .filter_map(|r| {
            if molecule.contains(&r.value) {
                Some(molecule.replacen(&r.value, &r.key, 1))
            } else {
                None
            }
        })
        .nth(0)
        .expect("no string")
}

fn part_2(replacements: &Vec<Replacement>, molecule: String) -> u32 {
    let mut i = 0;
    let mut molecule = molecule.clone();
    while &molecule != "e" {
        molecule = part_2_conversion(replacements, molecule);
        i += 1;
    }
    i
}

#[derive(Debug)]
struct Replacement {
    key: String,
    value: String,
}

impl From<&String> for Replacement {
    fn from(value: &String) -> Self {
        let (key, value) = value.split_once(" => ").unwrap();
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}
