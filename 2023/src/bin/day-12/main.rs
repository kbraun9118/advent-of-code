use std::{collections::HashMap, fmt::Display};

type Cache = HashMap<Row, usize>;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Row {
    springs: Vec<Status>,
    pattern: Vec<usize>,
}

impl Row {
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

fn solve(row: &Row, cache: &mut Cache) -> usize {
    if cache.contains_key(row) {
        return cache[row];
    }
    if row.pattern.is_empty() {
        if row.springs.iter().any(|s| *s == Status::Damaged) {
            cache.insert(row.clone(), 0);
            return 0;
        } else {
            cache.insert(row.clone(), 1);
            return 1;
        }
    }
    if row.springs.len() < row.pattern.iter().sum() {
        cache.insert(row.clone(), 0);
        return 0;
    }
    if row.springs.starts_with(&[Status::Operational]) {
        let ans = solve(
            &Row {
                springs: row.springs[1..].to_vec(),
                pattern: row.pattern.clone(),
            },
            cache,
        );
        cache.insert(row.clone(), ans);
        return ans;
    }
    if row.springs.starts_with(&[Status::Unknown]) {
        let operational = row.springs[1..].to_vec();
        let mut damaged = row.springs.clone();
        damaged[0] = Status::Damaged;

        let ans = solve(
            &Row {
                springs: operational,
                pattern: row.pattern.clone(),
            },
            cache,
        ) + solve(
            &Row {
                springs: damaged,
                pattern: row.pattern.clone(),
            },
            cache,
        );

        cache.insert(row.clone(), ans);
        return ans;
    }
    if row.springs.starts_with(&[Status::Damaged]) {
        if row.springs[0..row.pattern[0]]
            .iter()
            .all(|s| *s == Status::Damaged || *s == Status::Unknown)
        {
            if row.springs.len() == row.pattern[0] {
                cache.insert(row.clone(), 1);
                return 1;
            }
            if row.springs[row.pattern[0]] == Status::Operational
                || row.springs[row.pattern[0]] == Status::Unknown
            {
                let springs = row.springs[row.pattern[0] + 1..].to_vec();
                let pattern = row.pattern[1..].to_vec();
                let ans = solve(&Row { springs, pattern }, cache);
                cache.insert(row.clone(), ans);
                return ans;
            }
        }
        cache.insert(row.clone(), 0);
        return 0;
    }

    // should never get here
    cache.insert(row.clone(), 0);
    0
}

fn part_1(rows: &Vec<Row>, cache: &mut Cache) -> usize {
    let mut ans = 0;
    for row in rows {
        ans += solve(&row, cache);
    }
    // println!("{cache:#?})");
    ans
}

fn part_2(rows: &Vec<Row>, cache: &mut Cache) -> usize {
    let mut ans = 0;
    for row in rows {
        ans += solve(&row.expand(), cache);
    }
    ans
}

fn main() {
    let rows = aoc::read_input_lines("12")
        .into_iter()
        .map(Row::from)
        .collect::<Vec<_>>();

    let mut cache = Cache::new();

    aoc::print_part_1(part_1(&rows, &mut cache));
    aoc::print_part_2(part_2(&rows, &mut cache));
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
        let mut cache = Cache::new();
        assert_eq!(part_1(&get_test_input(), &mut cache), 21);
    }

    #[test]
    fn test_part_2() {
        let mut cache = Cache::new();
        assert_eq!(part_2(&get_test_input(), &mut cache), 525152);
    }
}
