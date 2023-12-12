#[derive(Debug)]
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

#[derive(Debug)]
struct Row {
    springs: Vec<Status>,
    pattern: Vec<usize>,
}

impl Row {}

impl From<String> for Row {
    fn from(value: String) -> Self {
        let (springs, pattern) = value.split_once(" ").unwrap();
        let springs = springs.chars().map(Status::from).collect();
        let pattern = pattern.split(",").map(|s| s.parse().unwrap()).collect();
        Self { springs, pattern }
    }
}

fn main() {
    let input = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#
        .lines()
        .map(String::from)
        .collect::<Vec<_>>();
    let rows = input.into_iter().map(Row::from).collect::<Vec<_>>();

    println!("{rows:?}");
}
