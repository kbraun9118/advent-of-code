use std::collections::{HashSet, VecDeque};

type Garden = aoc::Grid<GridTile>;
type Coord = aoc::Coord<usize>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GridTile {
    Plot,
    Rock,
    Start,
}

impl GridTile {
    fn is_rock(self) -> bool {
        if let GridTile::Rock = self {
            true
        } else {
            false
        }
    }
}

impl From<char> for GridTile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Plot,
            '#' => Self::Rock,
            'S' => Self::Start,
            _ => panic!("Invalid char {value}"),
        }
    }
}

fn step(garden: &Garden, steps: HashSet<Coord>) -> HashSet<Coord> {
    let mut next = HashSet::new();
    for step in steps {
        garden
            .cardinal_neighbors_coords(step)
            .into_iter()
            .filter(|s| !garden[*s].is_rock())
            .for_each(|s| {
                next.insert(s);
            });
    }
    next
}

const PART_2_STEPS: usize = 26501365;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Floor,
    Wall,
    Visited,
}

fn part_2(input: &str) -> usize {
    let mut start = 0;
    let mut map: Vec<_> = input
        .chars()
        .filter(|&c| c != '\n')
        .enumerate()
        .map(|(pos, c)| match c {
            '.' => Tile::Floor,
            '#' => Tile::Wall,
            'S' => {
                start = pos;
                Tile::Visited
            }
            _ => panic!(),
        })
        .collect();
    let width = input.lines().next().unwrap().chars().count();
    let height = map.len() / width;
    assert!(width == height && width % 2 == 1 && (PART_2_STEPS - width / 2) % width == 0);
    let mut open_set: VecDeque<_> = [(0, start / width, start % width)].into_iter().collect();
    let mut center_even = 0;
    let mut center_odd = 0;
    let mut corner_tiles = 0;
    while let Some((steps, y, x)) = open_set.pop_front() {
        *match (steps > width / 2, steps % 2 == 0) {
            (false, false) => &mut center_odd,
            (false, true) => &mut center_even,
            (true, _) => &mut corner_tiles,
        } += 1;
        let new_steps = steps + 1;
        for (new_y, new_x) in [
            (y + 1, x),
            (y, x + 1),
            (y.wrapping_sub(1), x),
            (y, x.wrapping_sub(1)),
        ] {
            if new_y >= height || new_x >= width {
                continue;
            }
            let new_pos = new_y * width + new_x;
            if map[new_pos] != Tile::Floor {
                continue;
            }
            map[new_pos] = Tile::Visited;
            open_set.push_back((new_steps, new_y, new_x));
        }
    }

    let radius = (PART_2_STEPS - width / 2) / width;
    let center_even_diamonds = (1 + radius / 2 * 2).pow(2);
    let center_odd_diamonds = ((1 + radius) / 2 * 2).pow(2);
    let corner_diamonds = (radius * 2 + 1).pow(2) / 4;
    center_even_diamonds
        * if PART_2_STEPS % 2 == 0 {
            center_even
        } else {
            center_odd
        }
        + center_odd_diamonds
            * if PART_2_STEPS % 2 == 0 {
                center_odd
            } else {
                center_even
            }
        + corner_diamonds * corner_tiles
}

fn part_1(garden: &Garden, distance: usize) -> usize {
    let mut next = garden
        .index_iter()
        .filter(|i| garden[*i] == GridTile::Start)
        .collect::<HashSet<_>>();

    for _ in 0..distance {
        next = step(garden, next);
    }

    next.len()
}

fn main() {
    let input = aoc::read_input_lines("21");
    let part_2_input = aoc::read_input_lines_raw("21");

    aoc::benchmark(|| {
        let garden = Garden::from(
            input
                .into_iter()
                .map(|l| l.chars().map(GridTile::from).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        );

        aoc::print_part_1(part_1(&garden, 64));
        aoc::print_part_2(part_2(&part_2_input));
    })
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_input() -> Garden {
        let input = r"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."
            .lines()
            .map(String::from)
            .collect::<Vec<_>>();
        Garden::from(
            input
                .into_iter()
                .map(|l| l.chars().map(GridTile::from).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        )
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_test_input(), 6), 16);
    }
}
