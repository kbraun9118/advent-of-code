use std::fmt::Display;

use rayon::prelude::*;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Status {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for Status {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => panic!("Invalid character"),
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Operational => '.',
                Self::Unknown => '?',
                Self::Damaged => '#',
            }
        )
    }
}

#[derive(Debug)]
struct Row {
    springs: Vec<Status>,
    pattern: Vec<usize>,
}

impl Row {
    fn is_valid(&self) -> bool {
        let pattern = self
            .springs
            .split(|v| *v == Status::Operational)
            .map(|s| s.len())
            .filter(|l| *l != 0)
            .collect::<Vec<_>>();

        self.pattern == pattern
    }

    fn all_springs(&self) -> Vec<Row> {
        all_springs(&self.springs)
            .into_iter()
            .map(|s| Row {
                springs: s,
                pattern: self.pattern.clone(),
            })
            .collect()
    }

    fn expand(&self) -> Row {
        let pattern = self
            .pattern
            .iter()
            .cloned()
            .cycle()
            .take(self.pattern.len() * 5)
            .collect();
        let springs = self
            .springs
            .iter()
            .cloned()
            .chain(vec![Status::Unknown].into_iter())
            .cycle()
            .take(self.springs.len() * 5 + 4)
            .collect();
        Self { pattern, springs }
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{} {}",
            self.springs
                .iter()
                .map(|s| format!("{s}"))
                .collect::<Vec<_>>()
                .join(""),
            self.pattern
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

impl From<String> for Row {
    fn from(value: String) -> Self {
        let (springs, pattern) = value.split_once(" ").unwrap();
        let springs = springs.chars().map(Status::from).collect();
        let pattern = pattern.split(",").map(|s| s.parse().unwrap()).collect();
        Self { springs, pattern }
    }
}

fn all_springs(springs: &Vec<Status>) -> Vec<Vec<Status>> {
    if let Some((i, _)) = springs
        .iter()
        .enumerate()
        .find(|(_, s)| **s == Status::Unknown)
    {
        let mut damaged = springs.clone();
        let mut operational = springs.clone();
        damaged[i] = Status::Damaged;
        operational[i] = Status::Operational;
        vec![all_springs(&damaged), all_springs(&operational)]
            .into_iter()
            .flatten()
            .collect()
    } else {
        vec![springs.clone()]
    }
}
fn part_1(rows: &Vec<Row>) -> usize {
    rows.iter()
        .map(|r| r.all_springs().iter().filter(|s| s.is_valid()).count())
        .sum()
}

fn part_2(rows: &Vec<Row>) -> usize {
    rows.par_iter()
        .map(|r| r.expand())
        .enumerate()
        .inspect(|(i, _)| println!("Expanded: {}", i + 1))
        .map(|(_, r)| r.all_springs().iter().filter(|s| s.is_valid()).count())
        .enumerate()
        .inspect(|(i, _)| println!("Calculated: {}", i + 1))
        .map(|(_, c)| c)
        .sum()
}

fn main() {
    let rows = aoc::read_input_lines("12")
        .into_iter()
        .map(Row::from)
        .collect::<Vec<_>>();

    aoc::print_part_1(part_1(&rows));
    aoc::print_part_2(part_2(&rows));
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_input() -> Vec<Row> {
        r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#
            .lines()
            .map(String::from)
            .map(Row::from)
            .collect::<Vec<_>>()
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_test_input()), 21);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_test_input()), 525152);
    }
}
