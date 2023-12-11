use core::panic;
use std::fmt::Display;

type Coord = aoc::Coord<usize>;

#[derive(Debug)]
struct Image {
    grid: Vec<Vec<Object>>,
    vertical_expansions: Vec<usize>,
    horizontal_expansions: Vec<usize>,
}

impl Image {
    fn distance(&self, from: Coord, to: Coord, expansion_factor: usize) -> usize {
        let vertical_expansions_crossed = (from.x.min(to.x)..from.x.max(to.x))
            .filter(|i| self.horizontal_expansions.contains(i))
            .count();
        let horizontal_expansions_crossed = (from.y.min(to.y)..from.y.max(to.y))
            .filter(|i| self.vertical_expansions.contains(i))
            .count();

        (from.x.max(to.x) - from.x.min(to.x))
            + (from.y.max(to.y) - from.y.min(to.y))
            + (vertical_expansions_crossed * expansion_factor)
            + (horizontal_expansions_crossed * expansion_factor)
    }

    fn all_pairs(&self) -> Vec<(Coord, Coord)> {
        let coords = self
            .grid
            .iter()
            .enumerate()
            .map(|(y, r)| {
                r.iter()
                    .enumerate()
                    .filter(|(_, s)| **s == Object::Galaxy)
                    .map(|(x, _)| Coord::from((x, y)))
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>();

        coords[0..coords.len() - 1]
            .iter()
            .enumerate()
            .map(|(i, l)| {
                coords[i + 1..]
                    .iter()
                    .filter(|o| l != *o)
                    .map(|r| (l.clone(), r.clone()))
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect()
    }
}

impl From<Vec<String>> for Image {
    fn from(value: Vec<String>) -> Self {
        let grid = value
            .iter()
            .map(|l| l.chars().map(Object::from).collect::<Vec<Object>>())
            .collect::<Vec<_>>();

        let vertical_expansions = grid
            .iter()
            .enumerate()
            .filter(|(_, r)| r.iter().all(|o| *o == Object::Space))
            .map(|(y, _)| y)
            .collect();

        let mut horizontal_expansions = vec![];

        for x in 0..grid[0].len() {
            if grid
                .iter()
                .map(|v| v[x].clone())
                .all(|o| o == Object::Space)
            {
                horizontal_expansions.push(x);
            }
        }

        Image {
            grid,
            vertical_expansions,
            horizontal_expansions,
        }
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.iter() {
            for o in row {
                write!(
                    f,
                    "{}",
                    match o {
                        Object::Space => '.',
                        Object::Galaxy => '#',
                    }
                )?;
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Object {
    Space,
    Galaxy,
}

impl From<char> for Object {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Space,
            '#' => Self::Galaxy,
            _ => panic!("Invalid character {value}"),
        }
    }
}

fn part_1(image: &Image) -> usize {
    image
        .all_pairs()
        .into_iter()
        .map(|(l, r)| image.distance(l, r, 1))
        .sum()
}

fn part_2(image: &Image) -> usize {
    image
        .all_pairs()
        .into_iter()
        .map(|(l, r)| image.distance(l, r, 999_999))
        .sum()
}

fn main() {
    let map = Image::from(aoc::read_input_lines("11"));

    aoc::print_part_1(part_1(&map));
    aoc::print_part_2(part_2(&map));
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_input() -> Image {
        Image::from(
            r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#
                .lines()
                .map(String::from)
                .collect::<Vec<_>>(),
        )
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_test_input()), 374);
    }

    #[test]
    fn test_part_2() {
        let image = get_test_input();
        let part_2_test: usize = image
            .all_pairs()
            .into_iter()
            .map(|(l, r)| image.distance(l, r, 99))
            .sum();
        assert_eq!(part_2_test, 8410);
    }
}
