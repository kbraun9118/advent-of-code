use core::panic;
use std::fmt::Display;

type Coord = aoc::Coord<usize>;

#[derive(Debug)]
struct Image(Vec<Vec<Object>>);

impl Image {
    fn expand(&self) -> Self {
        let mut vertical_expanded = self
            .0
            .iter()
            .map(|r| {
                if r.iter().all(|o| o == &Object::Space) {
                    vec![r.clone(), r.clone()]
                } else {
                    vec![r.clone()]
                }
            })
            .flatten()
            .collect::<Vec<_>>();

        let mut indecies_to_expand = vec![];

        for x in 0..vertical_expanded[0].len() {
            if vertical_expanded
                .iter()
                .map(|v| v[x].clone())
                .all(|o| o == Object::Space)
            {
                indecies_to_expand.push(x);
            }
        }

        for (i, x) in indecies_to_expand.iter().enumerate() {
            for row in vertical_expanded.iter_mut() {
                row.insert(i + x, Object::Space);
            }
        }

        Self(vertical_expanded)
    }
}

impl From<Vec<String>> for Image {
    fn from(value: Vec<String>) -> Self {
        Self(
            value
                .iter()
                .map(|l| l.chars().map(Object::from).collect())
                .collect(),
        )
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0.iter() {
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
    let image = image.expand();
    let coords = image
        .0
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

    println!("{coords:#?}");

    0
}

fn main() {
    let input = r#"...#......
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
        .collect::<Vec<_>>();
    let map = Image::from(input);

    aoc::print_part_1(part_1(&map));
}
