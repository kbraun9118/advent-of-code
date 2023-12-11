use std::{collections::HashSet, fmt::Display};

type Coord = aoc::Coord<usize>;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Pipe {
    Start,
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
    Horizontal,
    Vertical,
    Empty,
    Visited,
}

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        use Pipe::*;
        match value {
            '|' => Vertical,
            '-' => Horizontal,
            'F' => TopLeft,
            'J' => BottomRight,
            'L' => BottomLeft,
            '7' => TopRight,
            'S' => Start,
            '.' => Empty,
            c => panic!("Invalid character {c}"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Grid(Vec<Vec<Pipe>>);

impl Grid {
    fn find_loop(&self) -> Vec<Coord> {
        let mut current = self
            .0
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .find(|(_, v)| **v == Pipe::Start)
                    .map(|(x, _)| Coord { x, y })
            })
            .unwrap();

        let mut direction = Direction::Left;
        let mut inner_loop = vec![];

        while !inner_loop.contains(&current) {
            inner_loop.push(current);
            let (next_coord, next_direction) = self.next(current, direction);
            current = next_coord;
            direction = next_direction;
        }

        inner_loop
    }

    fn next(&self, current: Coord, direction: Direction) -> (Coord, Direction) {
        let current_pipe = self.0[current.y][current.x];
        use Direction::*;
        use Pipe::*;
        match (current_pipe, direction) {
            (Horizontal, Left) => ((current.x - 1, current.y).into(), Left),
            (Horizontal, Right) => ((current.x + 1, current.y).into(), Right),
            (Vertical, Up) => ((current.x, current.y - 1).into(), Up),
            (Vertical, Down) => ((current.x, current.y + 1).into(), Down),
            (TopLeft, Up) | (Start, Up) => ((current.x + 1, current.y).into(), Right),
            (TopLeft, Left) | (Start, Left) => ((current.x, current.y + 1).into(), Down),
            (TopRight, Up) => ((current.x - 1, current.y).into(), Left),
            (TopRight, Right) => ((current.x, current.y + 1).into(), Down),
            (BottomLeft, Down) => ((current.x + 1, current.y).into(), Right),
            (BottomLeft, Left) => ((current.x, current.y - 1).into(), Up),
            (BottomRight, Right) => ((current.x, current.y - 1).into(), Up),
            (BottomRight, Down) => ((current.x - 1, current.y).into(), Left),
            (current_pipe, direction) => {
                panic!("Invalid direction pipe combination: {current_pipe:?}, {direction:?}")
            }
        }
    }

    fn emtpy_neighbors(&self, coord: Coord) -> Vec<Coord> {
        let mut neighbors = vec![];
        for y in -1..=1 {
            for x in -1..=1 {
                if x != 0 || y != 0 {
                    let x = coord.x as i32 + x;
                    let y = coord.y as i32 + y;
                    if x > 0 && x < self.0[0].len() as i32 && y > 0 && y < self.0.len() as i32 {
                        if self.0[y as usize][x as usize] == Pipe::Empty {
                            neighbors.push((x as usize, y as usize).into());
                        }
                    }
                }
            }
        }
        neighbors
    }
}

impl From<Vec<String>> for Grid {
    fn from(value: Vec<String>) -> Self {
        Self(
            value
                .into_iter()
                .map(|l| l.chars().map(Pipe::from).collect())
                .collect(),
        )
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.0.iter() {
            for p in line {
                write!(
                    f,
                    "{}",
                    match p {
                        Pipe::Start => 'S',
                        Pipe::TopLeft => 'F',
                        Pipe::TopRight => '7',
                        Pipe::BottomRight => 'J',
                        Pipe::BottomLeft => 'L',
                        Pipe::Horizontal => '-',
                        Pipe::Vertical => '|',
                        Pipe::Empty => '.',
                        Pipe::Visited => 'O',
                    }
                )?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn part_1(grid: &Grid) -> usize {
    grid.find_loop().len() / 2
}

fn part_2(grid: &mut Grid) -> usize {
    let inner_loop = grid.find_loop();
    let max_y = grid.0.len();
    let max_x = grid.0[0].len();

    for y in 0..max_y {
        for x in 0..max_x {
            if !inner_loop.contains(&(x, y).into()) {
                grid.0[y][x] = Pipe::Empty;
            }
        }
    }

    let mut direction = Direction::Right;
    for Coord { x, y } in inner_loop.into_iter().rev() {
        use Direction::*;
        use Pipe::*;

        let mut to_add = vec![];
        // println!("Evaluating: {:?} {:?}", grid.0[y][x], direction.clone());
        match (grid.0[y][x], direction.clone()) {
            (Horizontal, Right) => {
                to_add.push((x, y + 1));
            }
            (Horizontal, Left) => {
                to_add.push((x, y - 1));
            }
            (Vertical, Up) => {
                to_add.push((x + 1, y));
            }
            (Vertical, Down) => {
                to_add.push((x - 1, y));
            }
            (TopRight, Right) => {
                direction = Down;
            }
            (TopRight, Up) => {
                direction = Left;
                to_add.push((x + 1, y));
                to_add.push((x, y - 1));
            }
            (BottomRight, Down) => {
                direction = Left;
            }
            (BottomRight, Right) => {
                direction = Up;
                to_add.push((x, y + 1));
                to_add.push((x + 1, y));
            }
            (BottomLeft, Left) => {
                direction = Up;
            }
            (BottomLeft, Down) => {
                direction = Right;
                to_add.push((x - 1, y));
                to_add.push((x, y + 1));
            }
            (TopLeft, Up) | (Start, Up) => {
                direction = Right;
            }
            (TopLeft, Left) | (Start, Left) => {
                direction = Down;
                to_add.push((x, y - 1));
                to_add.push((x - 1, y));
            }
            _ => panic!("invalid condition {:?} {:?}", grid.0[y][x], direction),
        }

        to_add.into_iter().for_each(|(x, y)| {
            if grid.0[y][x] == Empty {
                grid.0[y][x] = Visited;
            }
        })
    }


    let mut current = grid
        .0
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| c == &&Pipe::Visited)
                .map(move |(x, _)| Coord::from((x, y)))
        })
        .flatten()
        .collect::<HashSet<_>>();

    while !current.is_empty() {
        let mut next = HashSet::new();
        for coord in current {
            grid.0[coord.y][coord.x] = Pipe::Visited;
            grid.emtpy_neighbors(coord).into_iter().for_each(|c| {
                next.insert(c);
            });
        }
        current = next;
    }

    grid.0
        .iter()
        .flatten()
        .filter(|p| p == &&Pipe::Visited)
        .count()
}

fn main() {
    let mut grid = Grid::from(aoc::read_input_lines("10"));

    aoc::print_part_1(part_1(&grid));
    aoc::print_part_2(part_2(&mut grid));
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_input() -> Grid {
        r#"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"#
            .lines()
            .map(String::from)
            .collect::<Vec<_>>()
            .into()
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input()), 8);
    }
}
