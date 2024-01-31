use aoc_2015::lines_for_day;
use std::collections::{HashMap, HashSet};

fn main() {
    let commands = lines_for_day("06")
        .into_iter()
        .map(Command::from)
        .collect::<Vec<_>>();

    println!("Part One: {:?}", part_one(commands.clone()));
    println!("Part Two: {:?}", part_two(commands));
}

fn part_one(commands: Vec<Command>) -> usize {
    let mut points = HashSet::new();

    for command in commands {
        for point in command.square.points() {
            use CommandType::*;
            match command.command_type {
                On => {
                    points.insert(point);
                }
                Off => {
                    points.remove(&point);
                }
                Toggle => {
                    if points.contains(&point) {
                        points.remove(&point);
                    } else {
                        points.insert(point);
                    }
                }
            }
        }
    }

    points.len()
}

fn part_two(commands: Vec<Command>) -> u64 {
    let mut points: HashMap<Point, u64> = HashMap::new();

    for command in commands {
        for point in command.square.points() {
            use CommandType::*;
            match command.command_type {
                On => *points.entry(point).or_default() += 1,
                Off => {
                    points
                        .entry(point)
                        .and_modify(|brightness| {
                            if *brightness > 0 {
                                *brightness -= 1
                            }
                        })
                        .or_default();
                }
                Toggle => *points.entry(point).or_default() += 2,
            }
        }
    }

    points.into_values().sum()
}

#[derive(Debug, Copy, Clone)]
struct Square {
    start_x: u32,
    start_y: u32,
    end_x: u32,
    end_y: u32,
}

type Point = (u32, u32);

impl Square {
    fn points(self) -> Vec<Point> {
        let mut vec = vec![];
        for i in self.start_x..=self.end_x {
            for j in self.start_y..=self.end_y {
                vec.push((i, j));
            }
        }
        vec
    }
}

impl From<Vec<(&str, &str)>> for Square {
    fn from(vec: Vec<(&str, &str)>) -> Self {
        Square {
            start_x: vec[0].0.parse().expect("could not parse"),
            start_y: vec[0].1.parse().expect("could not parse"),
            end_x: vec[1].0.parse().expect("could not parse"),
            end_y: vec[1].1.parse().expect("could not parse"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum CommandType {
    On,
    Off,
    Toggle,
}

#[derive(Debug, Copy, Clone)]
struct Command {
    command_type: CommandType,
    square: Square,
}

impl From<String> for Command {
    fn from(string: String) -> Self {
        if string.starts_with("turn on") {
            let rest = string[8..]
                .split(" through ")
                .map(|s| s.split_once(',').expect("could not split"))
                .collect::<Vec<_>>();
            Self {
                command_type: CommandType::On,
                square: rest.into(),
            }
        } else if string.starts_with("turn off") {
            let rest = string[9..]
                .split(" through ")
                .map(|s| s.split_once(',').expect("could not split"))
                .collect::<Vec<_>>();
            Self {
                command_type: CommandType::Off,
                square: rest.into(),
            }
        } else {
            let rest = string[7..]
                .split(" through ")
                .map(|s| s.split_once(',').expect("could not split"))
                .collect::<Vec<_>>();
            Self {
                command_type: CommandType::Toggle,
                square: rest.into(),
            }
        }
    }
}
