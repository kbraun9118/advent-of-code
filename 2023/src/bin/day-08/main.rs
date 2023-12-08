use std::collections::HashMap;

#[derive(Debug)]
struct Network(HashMap<String, (String, String)>);

impl Network {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn add(&mut self, line: String) {
        let (id, nodes) = line.split_once(" = ").unwrap();
        let (left, right) = nodes.split_once(", ").unwrap();
        let left = left.replace("(", "");
        let right = right.replace(")", "");
        self.0.insert(id.to_string(), (left, right));
    }

    fn find_next<T: Into<String>>(&self, current: T, direction: Direction) -> String {
        let next = &self.0[&current.into()];
        match direction {
            Direction::Right => next.1.clone(),
            Direction::Left => next.0.clone(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'R' => Self::Right,
            'L' => Self::Left,
            _ => panic!("invalid character"),
        }
    }
}

fn part_1(network: &Network, directions: &Vec<Direction>) -> u32 {
    let mut steps = 0;
    let mut current = "AAA".to_string();
    let end = "ZZZ".to_string();
    let mut directions = directions.iter().cycle();
    while current != end {
        // println!("Current: {current}");
        if let Some(direction) = directions.next() {
            current = network.find_next(current.clone(), *direction);
            steps += 1;
        }
    }
    steps
}

fn part_2(network: &Network, directions: &Vec<Direction>) -> u64 {
    let paths = network
        .0
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(String::clone)
        .collect::<Vec<String>>();
    // let mut steps = 0;
    let directions = directions.iter().cycle();
    let mut path_lengths = vec![];

    for path in paths {
        let mut current_path = vec![];
        let mut current = path.to_string();
        while !current_path.contains(&current) {
            let mut directions = directions.clone();
            if let Some(direction) = directions.next() {
                current_path.push(current.clone());
                current = network.find_next(current, *direction);
            }
        }
        path_lengths.push(current_path.len());
    }

    path_lengths
        .into_iter()
        .fold(1u64, |acc, next| acc * next as u64)
}

fn parse_input(input: Vec<String>) -> (Network, Vec<Direction>) {
    let directions = input[0].chars().map(Direction::from).collect::<Vec<_>>();
    let network = input[2..].iter().fold(Network::new(), |mut acc, next| {
        acc.add(next.to_string());
        acc
    });
    (network, directions)
}

fn main() {
    let lines = aoc::read_input_lines("08");
    let (network, directions) = parse_input(lines);

    aoc::print_part_1(part_1(&network, &directions));
    aoc::print_part_2(part_2(&network, &directions));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#
            .lines()
            .map(String::from)
            .collect();
        let (network, directions) = parse_input(input);
        assert_eq!(part_1(&network, &directions), 6);
    }

    #[test]
    fn test_part_2() {
        let input = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#
            .lines()
            .map(String::from)
            .collect::<Vec<_>>();
        let (network, directions) = parse_input(input);
        assert_eq!(part_2(&network, &directions), 6);
    }
}
