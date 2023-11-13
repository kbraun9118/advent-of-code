use std::{
    collections::{HashMap, VecDeque},
    ops::Deref,
    vec,
};

use aoc_2015::lines_for_day;

fn main() {
    let lines = lines_for_day("day-13");
    let mut happiness = parse(lines);
    let all_arrangements = combos(happiness.keys().collect::<Vec<_>>());

    println!(
        "Part One: {}",
        compute_happiness(all_arrangements, &happiness)
    );

    happiness.values_mut().for_each(|m| {
        m.insert("Me".into(), 0);
    });

    happiness.insert(
        "Me".into(),
        happiness.keys().map(|p| (p.clone(), 0)).collect(),
    );

    let all_arrangements = combos(happiness.keys().collect::<Vec<_>>());

    println!(
        "Part Two: {}",
        compute_happiness(all_arrangements, &happiness)
    );
}

fn compute_happiness(arrangments: VecDeque<VecDeque<Person>>, happiness: &HappinessMap) -> i32 {
    arrangments
        .iter()
        .map(|a| {
            a.iter()
                .enumerate()
                .map(|(i, p)| {
                    if i == 0 {
                        happiness[p][&a[a.len() - 1]] + happiness[p][&a[1]]
                    } else if i == a.len() - 1 {
                        happiness[p][&a[0]] + happiness[p][&a[i - 1]]
                    } else {
                        happiness[p][&a[i - 1]] + happiness[p][&a[i + 1]]
                    }
                })
                .sum()
        })
        .max()
        .unwrap_or(0)
}

fn combos(people: Vec<&Person>) -> VecDeque<VecDeque<Person>> {
    if people.len() == 1 {
        vec![vec![people[0].clone()].into()].into()
    } else {
        let mut vecs: VecDeque<VecDeque<Person>> = VecDeque::new();
        for person in people.clone() {
            let mut without_person = combos(
                people
                    .clone()
                    .into_iter()
                    .filter(|o| person != *o)
                    .collect::<Vec<_>>(),
            );
            without_person
                .iter_mut()
                .for_each(|v| v.push_front(person.clone()));

            without_person
                .into_iter()
                .for_each(|v| vecs.push_front(v));
        }
        vecs
    }
}

fn parse(lines: Vec<String>) -> HappinessMap {
    let mut map: HappinessMap = HashMap::new();

    for line in lines {
        let (name, rest) = line.split_once(" would ").expect("could not split");
        let (num, rest) = if rest.starts_with("gain") {
            let (num, rest) = rest
                .trim_start_matches("gain ")
                .split_once(' ')
                .expect("could not split");
            (num.parse::<i32>().expect("could not parse"), rest)
        } else {
            let (num, rest) = rest
                .trim_start_matches("lose ")
                .split_once(' ')
                .expect("could not split");
            (-num.parse::<i32>().expect("could not parse"), rest)
        };

        let other = rest
            .to_string()
            .split_off(35)
            .trim_end_matches('.')
            .to_string();

        map.entry(name.into())
            .or_default()
            .entry(other.into())
            .or_insert(num);
    }

    map
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Person(String);

impl Deref for Person {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for Person {
    fn from(string: &str) -> Self {
        Self(string.to_string())
    }
}

impl From<String> for Person {
    fn from(string: String) -> Self {
        Self(string)
    }
}

type HappinessMap = HashMap<Person, HashMap<Person, i32>>;
