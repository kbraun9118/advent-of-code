use std::collections::{BinaryHeap, HashMap};

type Grid = aoc::Grid<Square>;
type Coord = aoc::Coord<usize>;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Square {
    Path,
    Forest,
    SlopeUp,
    SlopeDown,
    SlopeLeft,
    SlopeRight,
}

impl From<char> for Square {
    fn from(value: char) -> Self {
        use Square::*;
        match value {
            '.' => Path,
            '#' => Forest,
            '^' => SlopeUp,
            'v' => SlopeDown,
            '>' => SlopeRight,
            '<' => SlopeLeft,
            _ => panic!("Invalid char"),
        }
    }
}

fn get_longest_path(grid: &Grid) -> usize {
    let start = grid
        .index_iter()
        .filter(|c| c.y == 0)
        .filter(|c| grid[*c] == Square::Path)
        .next()
        .unwrap();
    let end = grid
        .index_iter()
        .filter(|c| c.y == grid.height() - 1)
        .filter(|c| grid[*c] == Square::Path)
        .next()
        .unwrap();

    0
}

fn get_neighbors(grid: &Grid, pos: Coord) -> Vec<Coord> {
    match grid[pos] {
        Square::Path => grid
            .cardinal_neighbors_coords(pos)
            .into_iter()
            .filter(|c| grid[*c] != Square::Forest)
            .collect(),
        Square::SlopeUp => vec![Coord {
            x: pos.x,
            y: pos.y - 1,
        }],
        Square::SlopeDown => vec![Coord {
            x: pos.x,
            y: pos.y + 1,
        }],
        Square::SlopeLeft => vec![Coord {
            x: pos.x - 1,
            y: pos.y,
        }],
        Square::SlopeRight => vec![Coord {
            x: pos.x + 1,
            y: pos.y,
        }],
        _ => panic!("invalid position"),
    }
}

fn get_test_input() -> Grid {
    r"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"
        .lines()
        .map(|s| s.chars().map(Square::from).collect::<Vec<_>>())
        .collect::<Vec<_>>()
        .into()
}

fn main() {
    let grid = get_test_input();

    aoc::print_part_1(get_longest_path(&grid));
}
