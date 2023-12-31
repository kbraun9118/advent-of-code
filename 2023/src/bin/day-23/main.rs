use std::collections::{BinaryHeap, HashMap, HashSet};

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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct SquareState {
    pos: Coord,
    state: isize,
}

impl PartialOrd for SquareState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.state.partial_cmp(&self.state)
    }
}

impl Ord for SquareState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.state.cmp(&self.state)
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

    let mut heap = BinaryHeap::new();
    heap.push(SquareState {
        pos: start,
        state: 0,
    });

    while let Some(SquareState { pos, state }) = heap.pop() {}

    0
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

    print!("{}", get_longest_path(&grid));
}
