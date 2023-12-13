type Grid = aoc::Grid<Terrain>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Terrain {
    Ash,
    Rock,
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

fn part_1(grids: &Vec<Grid>) -> usize {
    let mut sum = 0;

    for grid in grids {
        if let Some(reflection) = (1..grid.width()).find(|i| {
            (0..*i)
                .into_iter()
                .rev()
                .zip(*i..grid.width())
                .all(|(l, r)| grid.column(l) == grid.column(r))
        }) {
            sum += reflection;
        } else if let Some(reflection) = (1..grid.height()).find(|i| {
            (0..*i)
                .into_iter()
                .rev()
                .zip(*i..grid.height())
                .all(|(l, r)| grid.row(l) == grid.row(r))
        }) {
            sum += 100 * reflection;
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
}
