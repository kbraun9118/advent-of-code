use std::fmt::Display;

use aoc::Coord;

type Grid = aoc::Grid<Platform>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Platform {
    Rounded,
    Empty,
    Cubed,
}

impl Platform {
    fn is_rounded(self) -> bool {
        if let Self::Rounded = self {
            true
        } else {
            false
        }
    }
    // fn is_cubed(self) -> bool {
    //     if let Self::Cubed = self {
    //         true
    //     } else {
    //         false
    //     }
    // }

    fn is_empty(self) -> bool {
        if let Self::Empty = self {
            true
        } else {
            false
        }
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Cubed => '#',
                Self::Rounded => 'O',
                Self::Empty => '.',
            }
        )
    }
}

impl From<char> for Platform {
    fn from(value: char) -> Self {
        match value {
            'O' => Self::Rounded,
            '.' => Self::Empty,
            '#' => Self::Cubed,
            _ => panic!("invalid char"),
        }
    }
}

fn parse_grid(lines: Vec<String>) -> Grid {
    Grid::from(
        lines
            .iter()
            .map(|l| l.chars().map(Platform::from).collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    )
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    West,
    East,
}

fn move_platforms(grid: &Grid, direction: Direction) -> Grid {
    let mut grid = grid.clone();
    let mut index_iter = grid
        .index_iter()
        .filter(|p| grid[*p].is_rounded())
        .collect::<Vec<_>>();

    if direction == Direction::South || direction == Direction::East {
        index_iter.reverse();
    }

    for r in index_iter {
        let mut next: Coord<isize> = match direction {
            Direction::North => (r.x as isize, r.y as isize - 1),
            Direction::West => (r.x as isize - 1, r.y as isize),
            Direction::East => (r.x as isize + 1, r.y as isize),
            Direction::South => (r.x as isize, r.y as isize + 1),
        }
        .into();
        let mut previous = (r.x as isize, r.y as isize).into();
        while next.y > -1
            && next.y < grid.height() as isize
            && next.x > -1
            && next.x < grid.width() as isize
            && grid[(next.x as usize, next.y as usize)].is_empty()
        {
            previous = next;
            next = match direction {
                Direction::North => (next.x, next.y - 1),
                Direction::West => (next.x - 1, next.y),
                Direction::East => (next.x + 1, next.y),
                Direction::South => (next.x, next.y + 1),
            }
            .into();
        }
        grid[r] = Platform::Empty;
        grid[(previous.x as usize, previous.y as usize)] = Platform::Rounded;
    }

    grid
}

fn load(grid: &Grid) -> usize {
    grid.index_iter()
        .filter(|p| grid[*p].is_rounded())
        .map(|p| grid.height() - p.y)
        .sum()
}

fn part_1(grid: &Grid) -> usize {
    load(&move_platforms(grid, Direction::North))
}

fn part_2(grid: &Grid) -> usize {
    let mut grid = grid.clone();
    // let mut map: HashMap<Direction, usize> = vec![
    //     (Direction::North, 0),
    //     (Direction::West, 0),
    //     (Direction::South, 0),
    //     (Direction::East, 0),
    // ]
    // .into_iter()
    // .collect();
    let mut previous = vec![];

    for direction in vec![
        Direction::North,
        Direction::West,
        Direction::South,
        Direction::East,
    ]
    .into_iter()
    .cycle()
    .take(1_000_000_000)
    {
        grid = move_platforms(&grid, direction);
        previous.push((direction, grid.clone()));
        if previous.len() > 8
            && previous[previous.len() - 4..]
                .iter()
                .zip(previous[previous.len() - 8..previous.len() - 4].iter())
                .all(|(l, r)| l == r)
        {
            break;
        }
    }

    println!("{:?}", previous[previous.len() - 8..].iter());

    load(
        &previous
            .into_iter()
            .rev()
            .find(|(d, _)| *d == Direction::East)
            .map(|(_, l)| l)
            .unwrap(),
    )
}

fn get_input() -> Vec<String> {
    r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#
        .lines()
        .map(String::from)
        .collect::<Vec<_>>()
}

fn main() {
    let grid = parse_grid(aoc::read_input_lines("14"));

    aoc::print_part_1(part_1(&grid));
    aoc::print_part_2(part_2(&grid));
    // let mut grid = parse_grid(get_input());
    // for i in 0..3 {
    //     grid = move_platforms(&grid, Direction::North);
    //     println!("{grid}");
    //     grid = move_platforms(&grid, Direction::West);
    //     println!("{grid}");
    //     grid = move_platforms(&grid, Direction::South);
    //     println!("{grid}");
    //     grid = move_platforms(&grid, Direction::East);
    //     println!("After {} cycles: ", i + 1);
    //     println!("{grid}");
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_input() -> Vec<String> {
        r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#
            .lines()
            .map(String::from)
            .collect::<Vec<_>>()
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse_grid(get_input())), 136);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_grid(get_input())), 65);
    }
}
