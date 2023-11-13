use aoc_2015::lines_for_day;
use std::collections::HashMap;

type Point = (i32, i32);

fn main() {
    let input = lines_for_day("day-03").pop().unwrap();
    let house_visits = parse_input(input.clone());

    println!("Part One: {}", house_visits.len());
    let santa = input.chars().step_by(2).collect::<String>();
    let mut house_visits = parse_input(santa);
    let robo_santa = input.chars().skip(1).step_by(2).collect::<String>();
    let robo_visits = parse_input(robo_santa);
    house_visits.extend(robo_visits.iter());

    println!("Part Two: {}", house_visits.len());
}

fn parse_input(input: String) -> HashMap<Point, u32> {
    let mut house_visits: HashMap<Point, u32> = HashMap::new();
    house_visits.insert((0, 0), 1);
    let mut current = (0, 0);
    for c in input.chars() {
        current = match c {
            '>' => (current.0 + 1, current.1),
            'v' => (current.0, current.1 + 1),
            '<' => (current.0 - 1, current.1),
            _ => (current.0, current.1 - 1),
        };
        let v = house_visits.entry(current).or_default();
        *v += 1;
    }

    house_visits
}
