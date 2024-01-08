use std::collections::VecDeque;

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

fn get_longest_path<F: Fn(&Grid, Coord) -> Vec<Coord>>(grid: &Grid, neighbor_fn: F) -> usize {
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

    let mut paths = VecDeque::new();
    paths.push_back(vec![start]);
    let mut end_paths = vec![];

    while let Some(path) = paths.pop_front() {
        for neighbor in neighbor_fn(grid, *path.last().unwrap()) {
            if !path.contains(&neighbor) {
                if neighbor == end {
                    end_paths.push(path.clone());
                } else {
                    let mut next = path.clone();
                    next.push(neighbor);
                    paths.push_back(next);
                }
            }
        }
    }

    end_paths.into_iter().map(|v| v.len()).max().unwrap_or(0)
}

fn get_neighbors_part_1(grid: &Grid, pos: Coord) -> Vec<Coord> {
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

fn get_neighbors_part_2(grid: &Grid, pos: Coord) -> Vec<Coord> {
    match grid[pos] {
        Square::Forest => panic!("invalid position"),
        _ => grid
            .cardinal_neighbors_coords(pos)
            .into_iter()
            .filter(|c| grid[*c] != Square::Forest)
            .collect(),
    }
}

fn part_1(grid: &Grid) -> usize {
    get_longest_path(grid, get_neighbors_part_1)
}

fn part_2(grid: &Grid) -> usize {
    get_longest_path(grid, get_neighbors_part_2)
}

fn main() {
    let grid: Grid = aoc::read_input_lines("23")
        .into_iter()
        .map(|s| s.chars().map(Square::from).collect::<Vec<_>>())
        .collect::<Vec<_>>()
        .into();

    aoc::print_part_1(part_1(&grid));
    aoc::print_part_2(part_2(&grid));
}

#[cfg(test)]
mod test {
    use super::*;

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

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_test_input()), 94);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_test_input()), 154);
    }
}
