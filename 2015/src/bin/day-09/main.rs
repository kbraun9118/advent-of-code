use aoc_2015::lines_for_day;
use std::collections::{HashMap, VecDeque};
use std::ops::Deref;

type DistanceMap = HashMap<Location, Vec<Distance>>;

fn main() {
    let lines = lines_for_day("09");
    let distances = parse(lines);

    println!("Part One: {}", solve(&distances, true));
    println!("Part One: {}", solve(&distances, false));
}

fn parse(lines: Vec<String>) -> HashMap<Location, Vec<Distance>> {
    let mut map: HashMap<Location, Vec<Distance>> = HashMap::new();
    for line in lines {
        let (locations, distance) = line.split_once(" = ").expect("Could not split");
        let distance = distance.parse::<u32>().expect("Could not parse");
        let (first, second) = locations.split_once(" to ").expect("Could not split");
        map.entry(first.into())
            .or_default()
            .push(Distance::new(distance, second.into()));
        map.entry(second.into())
            .or_default()
            .push(Distance::new(distance, first.into()));
    }
    map
}

fn solve(map: &DistanceMap, is_min: bool) -> u32 {
    let mut distance = if is_min { u32::MAX } else { u32::MIN };
    for (loc, distances) in map {
        for dist in distances {
            let mut locations = VecDeque::new();
            locations.push_back(loc.clone());
            locations.push_back(dist.location.clone());
            let mut map = map.clone();
            map.remove(loc);
            distance = add_distance(is_min, distance, dist.value, &map, &locations)
        }
    }
    distance
}

fn solve_iter(map: &DistanceMap, locations: &VecDeque<Location>, is_min: bool) -> u32 {
    if map.len() == 1 {
        0
    } else {
        let mut distance = if is_min { u32::MAX } else { u32::MIN };
        let current_loc = locations.back().unwrap();
        for dist in map[current_loc]
            .iter()
            .filter(|d| !locations.contains(&d.location))
        {
            let mut map = map.clone();
            map.remove(current_loc);
            let mut locations = locations.clone();
            locations.push_back(dist.location.clone());
            distance = add_distance(is_min, distance, dist.value, &map, &locations)
        }
        distance
    }
}

fn add_distance(
    is_min: bool,
    distance: u32,
    dist: u32,
    map: &HashMap<Location, Vec<Distance>>,
    locations: &VecDeque<Location>,
) -> u32 {
    if is_min {
        distance.min(dist + solve_iter(map, locations, is_min))
    } else {
        distance.max(dist + solve_iter(map, locations, is_min))
    }
}

#[derive(Debug, Clone)]
struct Distance {
    value: u32,
    location: Location,
}

impl Distance {
    fn new(value: u32, location: Location) -> Self {
        Distance { value, location }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Location(String);

impl Deref for Location {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Into<String>> From<T> for Location {
    fn from(string: T) -> Self {
        Self(string.into())
    }
}
