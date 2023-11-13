#![feature(exclusive_range_pattern)]
use std::{collections::HashSet, vec};

use aoc_2015::lines_for_day;

fn main() {
    let mut lines = lines_for_day("day-17")
        .into_iter()
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    lines.sort();

    let containers = lines
        .into_iter()
        .enumerate()
        .map(|(id, capacity)| Container { id, capacity })
        .collect::<Vec<Container>>();

    let mut set = HashSet::new();

    sum_to(&containers, 150, &mut set, vec![]);

    println!("Part 1: {}", set.len());

    let min = set.iter().map(Vec::len).min().unwrap_or(0);

    println!(
        "Part 2: {}",
        set.iter().map(Vec::len).filter(|l| *l == min).count()
    );
}

fn sum_to(
    rest: &[Container],
    sum: i32,
    set: &mut HashSet<Vec<Container>>,
    mut path: Vec<Container>,
) -> i32 {
    match (rest, sum) {
        (_, 0) => {
            set.insert(path);
            1
        }
        ([rest], sum) if rest.capacity == sum => {
            path.push(*rest);
            set.insert(path);
            1
        }
        (_, i32::MIN..0) | ([], _) | ([_], _) => 0,
        (rest, sum) => {
            (0..rest.len() - 1)
                .map(|i| {
                    let mut path_with = path.clone();
                    path_with.push(rest[i]);
                    sum_to(&rest[i + 1..], sum - rest[i].capacity, set, path_with)
                })
                .sum::<i32>()
                + sum_to(&rest[1..], sum, set, path)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Container {
    id: usize,
    capacity: i32,
}
