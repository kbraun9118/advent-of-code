use std::{fmt::Display, fs::read};

pub fn read_input(day: &str) -> Result<Vec<String>, Error> {
    Ok(
        String::from_utf8(read(format!("../input/2025/{}/input.txt", day))?)?
            .lines()
            .map(String::from)
            .collect(),
    )
}

pub fn read_example_input(day: &str) -> Result<Vec<String>, Error> {
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

#[derive(Debug)]
pub enum Error {
    Custom(String),
    IoError(std::io::Error),
    FromUtf8(std::string::FromUtf8Error),
    ParseInt(std::num::ParseIntError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Error::Custom(error) => error.clone(),
                Error::IoError(error) => format!("Io Error: {}", error),
                Error::FromUtf8(error) => format!("UTF 8 Error: {}", error),
                Self::ParseInt(error) => format!("Parse Int Error: {}", error),
            }
        )
    }
}

impl std::error::Error for Error {}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::Custom(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(value: std::string::FromUtf8Error) -> Self {
        Self::FromUtf8(value)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(value: std::num::ParseIntError) -> Self {
        Self::ParseInt(value)
    }
}
