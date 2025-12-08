use std::fs::read;

pub type Error = Box<dyn std::error::Error>;

pub fn read_input(day: &str) -> Result<Vec<String>, Error> {
    Ok(
        String::from_utf8(read(format!("../input/2025/{}/input.txt", day))?)?
            .lines()
            .map(String::from)
            .collect(),
    )
}

pub fn read_input_example(day: &str) -> Result<Vec<String>, Error> {
    Ok(
        String::from_utf8(read(format!("examples/{}/input.txt", day))?)?
            .lines()
            .map(String::from)
            .collect(),
    )
}

#[macro_export]
macro_rules! print_output {
    // One argument
    ($a:expr) => {
        println!("Part 1: {:?}", $a);
    };

    // Two arguments
    ($a:expr, $b:expr) => {
        println!("Part 1: {:?}\nPart 2: {:?}", $a, $b);
    };
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}
