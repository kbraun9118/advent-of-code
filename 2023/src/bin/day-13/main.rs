type Grid = aoc::Grid<Terrain>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Terrain {
    Ash,
    Rock,
}

impl Terrain {
    fn flip(self) -> Self {
        match self {
            Terrain::Ash => Terrain::Rock,
            Terrain::Rock => Terrain::Ash,
        }
    }
}

impl From<char> for Terrain {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Rock,
            '.' => Self::Ash,
            _ => panic!("invalid character"),
        }
    }
}

fn solve_grid(grid: &Grid, to_skip: Option< usize >) -> usize {
    if let Some(reflection) = (1..grid.width())
        .filter(|i| Some( *i ) != to_skip)
        .find(|i| {
        (0..*i)
            .into_iter()
            .rev()
            .zip(*i..grid.width())
            .all(|(l, r)| grid.column(l) == grid.column(r))
    }) {
        reflection
    } else if let Some(reflection) = (1..grid.height())
        .filter(|i| Some( *i * 100 ) != to_skip)
        .find(|i| {
        (0..*i)
            .into_iter()
            .rev()
            .zip(*i..grid.height())
            .all(|(l, r)| grid.row(l) == grid.row(r))
    }) {
        100 * reflection
    } else {
        0
    }
}

fn part_1(grids: &Vec<Grid>) -> usize {
    let mut sum = 0;

    for grid in grids {
        sum += solve_grid(&grid, None);
    }

    sum
}

fn part_2(grids: &Vec<Grid>) -> usize {
    let mut grids = grids.clone();
    let mut sum = 0;
    for grid in grids.iter_mut() {
        let skip = Some(solve_grid(&grid, None));
        'grid: for i in grid.index_iter().collect::<Vec<_>>() {
            grid[i] = grid[i].flip();
            if let s @ 1..  = solve_grid(&grid, skip) {
                sum += s;
                break 'grid;
            }
            grid[i] = grid[i].flip()
        }
    }
     sum
}

fn main() {
    let grids = aoc::read_input_lines("13")
        .split(|l| l == "")
        .map(|s| {
            s.iter()
                .map(|r| r.chars().map(Terrain::from).collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .map(Grid::from)
        .collect::<Vec<_>>();

    aoc::print_part_1(part_1(&grids));
    aoc::print_part_2(part_2(&grids));
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_input() -> Vec<Grid> {
        r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#
            .lines()
            .map(String::from)
            .collect::<Vec<_>>()
            .split(|l| l == "")
            .map(|s| {
                s.iter()
                    .map(|r| r.chars().map(Terrain::from).collect::<Vec<_>>())
                    .collect::<Vec<_>>()
            })
            .map(Grid::from)
            .collect::<Vec<_>>()
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_test_input()), 405);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_test_input()), 400);
    }
}
