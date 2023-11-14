#![allow(unused)]

use std::fmt::Display;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::ops::RangeInclusive;
use std::path::Path;

fn main() {
    let reader = BufReader::new(
        File::open(Path::new("src/bin/day-17/input.txt")).unwrap());

    let input: Vec<Input> = reader.lines()
        .map(|line| line.unwrap())
        .map(|string| string.into())
        .collect();

    input.iter().for_each(|input| println!("{:?}", input));
    let mut ground = Ground::from(input);
    run_to_end(&mut ground);
    println!("{}", ground);
    println!("{}", ground.count_water());
}

fn run_to_end(ground: &mut Ground) {
    let mut out = ground.tick(vec![ground.get_source()]);

//    while !out.is_empty() {
//        out = ground.tick(out);
//    }
    for _ in 0..300 {
        out = ground.tick(out);
    }
}

#[derive(Debug)]
enum Input {
    // y is constant
    Width((usize, RangeInclusive<usize>)),
    // x is constant
    Height((usize, RangeInclusive<usize>)),
}

impl Input {
    fn get_window(inputs: &Vec<Input>) -> ((usize, usize), (usize, usize)) {
        let mut min_x: usize = usize::max_value();
        let mut max_x = 0;
        let mut min_y = usize::max_value();
        let mut max_y: usize = 0;

        for input in inputs {
            match input {
                Input::Height((x, range)) => {
                    if *x < min_x {
                        min_x = *x;
                    }
                    if *x > max_x {
                        max_x = *x;
                    }
                    if *range.start() < min_y {
                        min_y = *range.start()
                    }
                    if *range.end() > max_y {
                        max_y = *range.end()
                    }
                }
                Input::Width((y, range)) => {
                    if *y < min_y {
                        min_y = *y;
                    }
                    if *y > max_y {
                        max_y = *y;
                    }
                    if *range.start() < min_x {
                        min_x = *range.start()
                    }
                    if *range.end() > max_x {
                        max_x = *range.end()
                    }
                }
            }
        }

        ((min_x, max_x), (min_y, max_y))
    }
}

impl From<String> for Input {
    fn from(string: String) -> Self {
        let split: Vec<&str> = string.split(", ").collect();
        let constant: usize = split[0][2..].parse().unwrap();
        let range: Vec<&str> = split[1][2..].split("..").collect();
        let start_range: usize = range[0].parse().unwrap();
        let end_range: usize = range[1].parse().unwrap();

        match split[0].chars().next().unwrap() {
            'x' => Input::Height((constant, start_range..=end_range)),
            'y' => Input::Width((constant, start_range..=end_range)),
            _ => panic!("Invalid input")
        }
    }
}

#[derive(Debug, Clone)]
enum Tile {
    Source,
    Sand(Option<Water>),
    Clay,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        Ok(match self {
            Tile::Source => write!(f, "+")?,
            Tile::Sand(Some(water)) => write!(f, "{}", water)?,
            Tile::Sand(None) => write!(f, ".")?,
            Tile::Clay => write!(f, "#")?
        })
    }
}

#[derive(Debug, Clone)]
enum Water {
    Rest,
    Flowing,
}

impl Display for Water {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        Ok(match self {
            Water::Flowing => write!(f, "|")?,
            Water::Rest => write!(f, "~")?
        })
    }
}

#[derive(Debug)]
struct Ground(Vec<Vec<Tile>>);

impl Ground {
    fn tick(&mut self, inputs: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
        let mut returned = vec![];

        for (x, y) in inputs {
            if y + 1 == self.0.len() {
                continue;
            }

            match self.0[y + 1][x] {
                Tile::Sand(None) => {
                    returned.push((x, y + 1));
                    self.0[y + 1][x] = Tile::Sand(Some(Water::Flowing))
                }
                Tile::Sand(Some(_)) | Tile::Clay => {
                    let mut overflow = false;
                    let mut i = 1;
                    loop {
                        if let Tile::Sand(None) = self.0[y + 1][x + i] {
                            overflow = true;
                            break;
                        }
                        if let Tile::Clay = self.0[y][x + i] {
                            break;
                        }
                        i += 1;
                    }
                    i = 1;
                    loop {
                        if let Tile::Sand(None) = self.0[y + 1][x - i] {
                            overflow = true;
                            break;
                        }
                        if let Tile::Clay = self.0[y][x - i] {
                            break;
                        }
                        i += 1;
                    }
                    if overflow {
                        i = 0;
                        loop {
                            if let Tile::Sand(None) = self.0[y + 1][x + i] {
                                returned.push((x + i, y + 1));
                                self.0[y + 1][x + i] = Tile::Sand(Some(Water::Flowing));
                                self.0[y][x + i] = Tile::Sand(Some(Water::Flowing));
                                break;
                            }
                            if let Tile::Clay = self.0[y][x + i] {
                                break;
                            } else {
                                self.0[y][x + i] = Tile::Sand(Some(Water::Flowing));
                                i += 1;
                            }
                        }
                        i = 0;
                        loop {
                            if let Tile::Sand(None) = self.0[y + 1][x - i] {
                                returned.push((x - i, y + 1));
                                self.0[y + 1][x - i] = Tile::Sand(Some(Water::Flowing));
                                self.0[y][x - i] = Tile::Sand(Some(Water::Flowing));
                                break;
                            }
                            if let Tile::Clay = self.0[y][x - i] {
                                break;
                            } else {
                                self.0[y][x - i] = Tile::Sand(Some(Water::Flowing));
                                i += 1;
                            }
                        }
                    } else {
                        returned.push((x, y - 1));
                        i = 0;
                        loop {
                            if let Tile::Clay = self.0[y][x + i] {
                                break;
                            } else {
                                self.0[y][x + i] = Tile::Sand(Some(Water::Rest));
                                i += 1;
                            }
                        }
                        i = 0;
                        loop {
                            if let Tile::Clay = self.0[y][x - i] {
                                break;
                            } else {
                                self.0[y][x - i] = Tile::Sand(Some(Water::Rest));
                                i += 1;
                            }
                        }
                    }
                    println!("Overflow is: {}", overflow)
                }
                _ => panic!("Cannot Reach here")
            }
        }

        returned
    }

    fn get_source(&self) -> (usize, usize) {
        for (x, tile) in self.0[0].iter().enumerate() {
            if let Tile::Source = tile {
                return (x, 0);
            }
        }
        panic!("No Source");
    }

    fn count_water(&self) -> usize {
        self.0
            .iter()
            .flat_map(|item| item)
            .filter(|item| {
                if let Tile::Sand(Some(_)) = item {
                    true
                } else { false }
            })
            .count()
    }
}

impl From<Vec<Input>> for Ground {
    fn from(inputs: Vec<Input>) -> Self {
        let ((offset, max_x), (_, max_y)) = Input::get_window(&inputs);
        let offset = offset - 1;

        let mut ground = vec![vec![Tile::Sand(None); max_x - offset + 2]; max_y + 1];

        ground[0][500 - offset] = Tile::Source;

        for input in inputs {
            match input {
                Input::Width((y, range)) => {
                    for x in range {
                        ground[y][x - offset] = Tile::Clay;
                    }
                }
                Input::Height((x, range)) => {
                    for y in range {
                        ground[y][x - offset] = Tile::Clay;
                    }
                }
            }
        }

        Ground(ground)
    }
}

impl Display for Ground {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for y in 0..self.0.len() {
            for x in 0..self.0[0].len() {
                write!(f, "{}", self.0[y][x])?;
            }
            writeln!(f)?
        }
        Ok(())
    }
}
