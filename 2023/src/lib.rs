use std::fmt::Display;

pub fn read_input_lines(day: &'static str) -> Vec<String> {
    let file = std::fs::read_to_string(format!("./src/bin/day-{}/input.txt", day))
        .expect("Could not find file");

    let mut lines = file.lines().map(|s| s.to_string()).collect::<Vec<_>>();
    if lines.last().unwrap() == "" {
        lines.pop();
        lines
    } else {
        lines
    }
}

pub fn print_part_1<T: Display>(part_1: T) {
    println!("Part 1: {}", part_1);
}

pub fn print_part_2<T: Display>(part_2: T) {
    println!("Part 2: {}", part_2);
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coord<T> {
    pub x: T,
    pub y: T,
}

impl<T> From<(T, T)> for Coord<T> {
    fn from((x, y): (T, T)) -> Self {
        Self { x, y }
    }
}
