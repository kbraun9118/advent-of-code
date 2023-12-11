use std::fmt::Display;

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

fn part_2(grid: &Grid) -> usize {
    let mut inner_loop = grid.find_loop();

    inner_loop.push(inner_loop[0].clone());

    let area = inner_loop
        .windows(2)
        .map(|w| {
            let l = w[0];
            let r = w[1];
            l.x as isize * r.y as isize - r.x as isize * l.y as isize
        })
        .sum::<isize>()
        .abs() as usize
        / 2;

    area - ((inner_loop.len() - 1) / 2) + 1
}

fn main() {
    let mut grid = Grid::from(aoc::read_input_lines("10"));

    aoc::print_part_1(part_1(&grid));
    aoc::print_part_2(part_2(&mut grid));
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_input_part_1() -> Grid {
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

    fn get_input_part_2() -> Grid {
        r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"#
            .lines()
            .map(String::from)
            .collect::<Vec<_>>()
            .into()
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input_part_1()), 8);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_input_part_2()), 10);
    }
}
