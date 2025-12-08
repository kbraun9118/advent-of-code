use std::{collections::HashSet, hash::Hash};

use aoc_2025::{Error, Position, print_output, read_input};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Posibilities {
    position: Position,
    timelines: usize,
}

#[derive(Debug, Clone)]
struct Manifold {
    diagram: Vec<Vec<char>>,
    current_positions: HashSet<Posibilities>,
    splits: usize,
    timelines: usize,
}

impl Manifold {
    fn new(input: Vec<String>) -> Result<Self, Error> {
        let diagram = input
            .into_iter()
            .map(|l| l.chars().collect())
            .collect::<Vec<Vec<_>>>();

        let start = diagram.iter().enumerate().find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, c)| {
                if *c == 'S' {
                    Some(Posibilities {
                        position: Position::new(x, y),
                        timelines: 1,
                    })
                } else {
                    None
                }
            })
        });

        Ok(Self {
            diagram,
            current_positions: [start.ok_or("could not find a start value")?].into(),
            splits: 0,
            timelines: 0,
        })
    }

    fn tick(&mut self) -> bool {
        let mut current = HashSet::new();
        std::mem::swap(&mut self.current_positions, &mut current);

        for mut posibility in current {
            let Position { x, y } = posibility.position;
            posibility.position.y += 1;
            if let Some(c) = self.diagram.get(y + 1).and_then(|row| row.get(x)).copied() {
                if c == '.' {
                    self.diagram[y + 1][x] = '|';
                    self.current_positions.insert(posibility.clone());
                }
                if c == '^' {
                    self.splits += 1;
                    if self.diagram.get(y + 1).and_then(|r| r.get(x + 1)).is_some() {
                        self.diagram[y + 1][x + 1] = '|';
                        let mut posibility = posibility.clone();
                        posibility.position.x += 1;

                        if let Some(mut added) = self
                            .current_positions
                            .extract_if(|p| {
                                p.position.x == posibility.position.x
                                    && p.position.y == posibility.position.y
                            })
                            .next()
                        {
                            added.timelines += posibility.timelines;
                            self.current_positions.insert(added);
                        } else {
                            self.current_positions.insert(posibility);
                        }
                    }
                    if self.diagram.get(y + 1).and_then(|r| r.get(x - 1)).is_some() {
                        self.diagram[y + 1][x - 1] = '|';
                        let mut posibility = posibility.clone();
                        posibility.position.x -= 1;

                        if let Some(mut added) = self
                            .current_positions
                            .extract_if(|p| {
                                p.position.x == posibility.position.x
                                    && p.position.y == posibility.position.y
                            })
                            .next()
                        {
                            added.timelines += posibility.timelines;
                            self.current_positions.insert(added);
                        } else {
                            self.current_positions.insert(posibility);
                        }
                    }
                }
                if c == '|' {
                    let mut added = self
                        .current_positions
                        .extract_if(|p| p.position.x == x && p.position.y == y + 1)
                        .next()
                        .expect("There should be a value here");
                    added.timelines += posibility.timelines;

                    self.current_positions.insert(added);
                }
            } else {
                self.timelines += posibility.timelines;
            }
        }

        self.current_positions.len() > 0
    }
}

fn main() -> Result<(), Error> {
    let input = read_input("07")?;
    let mut manifold = Manifold::new(input)?;

    while manifold.tick() {}

    print_output!(manifold.splits, manifold.timelines);

    Ok(())
}
