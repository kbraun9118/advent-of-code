use core::panic;
use std::collections::HashSet;

type Cave = aoc::Grid<Tile>;
type LightGrid = aoc::Grid<HashSet<Direction>>;
type Coord = aoc::Coord<usize>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    // '/'
    RightMirror,
    // '\',
    LeftMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        use Tile::*;
        match value {
            '.' => Empty,
            '/' => RightMirror,
            '\\' => LeftMirror,
            '-' => HorizontalSplitter,
            '|' => VerticalSplitter,
            _ => panic!("invalid char"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl Direction {
    fn intersects(self, tile: Tile) -> Vec<Direction> {
        use Direction::*;
        use Tile::*;
        match (self, tile) {
            (_, Empty)
            | (Left, HorizontalSplitter)
            | (Right, HorizontalSplitter)
            | (Up, VerticalSplitter)
            | (Down, VerticalSplitter) => vec![self],
            (_, HorizontalSplitter) => vec![Right, Left],
            (_, VerticalSplitter) => vec![Up, Down],
            (Up, RightMirror) | (Down, LeftMirror) => vec![Right],
            (Down, RightMirror) | (Up, LeftMirror) => vec![Left],
            (Left, RightMirror) | (Right, LeftMirror) => vec![Down],
            (Right, RightMirror) | (Left, LeftMirror) => vec![Up],
        }
    }
}

fn create_light_grid(
    cave: &Cave,
    starting_coord: Coord,
    starting_direction: Direction,
) -> LightGrid {
    use Direction::*;
    let mut current = vec![(starting_coord, starting_direction)];
    let mut light_grid = LightGrid::new_default(cave.height(), cave.width());

    while !current.is_empty() {
        let mut next = vec![];
        for (Coord { x, y }, direction) in current {
            if !light_grid[(x, y)].contains(&direction) {
                light_grid[(x, y)].insert(direction);
                for next_direction in direction.intersects(cave[(x, y)]) {
                    match next_direction {
                        Right => {
                            if x < light_grid.width() - 1 {
                                next.push((Coord { x: x + 1, y }, Right));
                            }
                        }
                        Left => {
                            if x > 0 {
                                next.push((Coord { x: x - 1, y }, Left));
                            }
                        }
                        Down => {
                            if y < light_grid.height() - 1 {
                                next.push((Coord { x, y: y + 1 }, Down))
                            }
                        }
                        Up => {
                            if y > 0 {
                                next.push((Coord { x, y: y - 1 }, Up))
                            }
                        }
                    }
                }
            }
        }
        current = next;
    }

    light_grid
}

fn create_cave(input: Vec<String>) -> Cave {
    input
        .iter()
        .map(|l| l.chars().map(Tile::from).collect::<Vec<_>>())
        .collect::<Vec<_>>()
        .into()
}

fn energy_level(light_grid: &LightGrid) -> usize {
    light_grid.iter().filter(|s| s.len() > 0).count()
}

fn part_1(cave: &Cave) -> usize {
    let light_grid = create_light_grid(&cave, (0, 0).into(), Direction::Right);
    energy_level(&light_grid)
}

fn part_2(cave: &Cave) -> usize {
    vec![
        (0..cave.width())
            .map(|i| {
                vec![
                    (Coord { x: i, y: 0 }, Direction::Down),
                    (
                        Coord {
                            x: i,
                            y: cave.height() - 1,
                        },
                        Direction::Up,
                    ),
                ]
            })
            .flatten()
            .collect::<Vec<_>>(),
        (0..cave.height())
            .map(|i| {
                vec![
                    (Coord { x: 0, y: i }, Direction::Right),
                    (
                        Coord {
                            x: cave.width() - 1,
                            y: i,
                        },
                        Direction::Left,
                    ),
                ]
            })
            .flatten()
            .collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
    .map(|(coord, direction)| create_light_grid(cave, coord, direction))
    .map(|lg| energy_level(&lg))
    .max()
    .unwrap()
}

fn main() {
    let cave = create_cave(aoc::read_input_lines("16"));

    aoc::print_part_1(part_1(&cave));
    aoc::print_part_2(part_2(&cave));
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_input() -> Cave {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."
            .lines()
            .map(String::from)
            .collect::<Vec<_>>();

        create_cave(input)
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input()), 46);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_input()), 51);
    }
}
