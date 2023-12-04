use std::collections::{HashMap, HashSet};

use aoc::Coord;

#[derive(Debug)]
struct PartList {
    parts: Vec<Part>,
}

impl From<Vec<String>> for PartList {
    fn from(lines: Vec<String>) -> Self {
        let mut current = String::new();
        let mut adjacents = vec![];
        let mut parts = vec![];
        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch.is_numeric() {
                    current.push(ch);
                    get_neighbors(&lines, x, y)
                        .into_iter()
                        .map(|c| (c, lines[c.y as usize].chars().nth(c.x as usize).unwrap()))
                        .for_each(|n| adjacents.push(n));
                } else {
                    if adjacents.len() > 0 {
                        parts.push(Part {
                            id: current.parse().unwrap(),
                            adjacents,
                        });
                        adjacents = vec![];
                    }
                    current = String::new();
                }
            }
            if adjacents.len() > 0 {
                parts.push(Part {
                    id: current.parse().unwrap(),
                    adjacents,
                });
                adjacents = vec![];
            }
            current = String::new();
        }

        Self { parts }
    }
}

#[derive(Debug)]
struct Part {
    id: u32,
    adjacents: Vec<(Coord, char)>,
}

fn get_neighbors(lines: &Vec<String>, x: usize, y: usize) -> Vec<Coord> {
    let mut neighbors = vec![];
    for nx in -1i32..=1 {
        for ny in -1i32..=1 {
            if nx != 0 || ny != 0 {
                let nx = nx + x as i32;
                let ny = ny + y as i32;
                if nx >= 0
                    && (nx as usize) < lines[0].len()
                    && ny >= 0
                    && (ny as usize) < lines.len()
                {
                    let ch = lines[ny as usize].chars().nth(nx as usize).unwrap();
                    if !ch.is_numeric() && ch != '.' {
                        neighbors.push((nx as u32, ny as u32).into())
                    }
                }
            }
        }
    }
    neighbors
}

fn part_1(part_list: &PartList) -> u32 {
    part_list.parts.iter().map(|p| p.id).sum()
}

fn part_2(part_list: &PartList) -> u32 {
    let mut map: HashMap<Coord, HashSet<u32>> = HashMap::new();
    for p in &part_list.parts {
        for (coord, _) in p.adjacents.iter().filter(|(_, ch)| *ch == '*') {
            map.entry(*coord).or_default().insert(p.id);
        }
    }
    map.values()
        .into_iter()
        .filter(|s| s.len() == 2)
        .map(|s| s.into_iter().fold(1, |acc, next| acc * *next))
        .sum()
}

fn main() {
    let part_list = PartList::from(aoc::read_input_lines("03"));
    aoc::print_part_1(part_1(&part_list));
    aoc::print_part_2(part_2(&part_list));
}

#[cfg(test)]
mod test {
    use crate::{part_1, part_2, PartList};

    fn input() -> PartList {
        r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#
            .lines()
            .map(String::from)
            .collect::<Vec<_>>()
            .into()
    }

    #[test]
    fn test_part_1() {
        let part_list = input();
        assert_eq!(part_1(&part_list), 4361);
    }

    #[test]
    fn test_part_2() {
        let part_list = input();
        assert_eq!(part_2(&part_list), 467835);
    }
}
