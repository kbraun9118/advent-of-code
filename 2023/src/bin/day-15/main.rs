use core::panic;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Lens {
    label: String,
    operation: Operation,
    focal_length: usize,
    hash: usize,
}

impl From<&str> for Lens {
    fn from(value: &str) -> Self {
        if value.contains("-") {
            let label = value.split("-").nth(0).unwrap().to_string();
            let hash = hash(&label);

            Self {
                label,
                operation: Operation::Minus,
                focal_length: 0,
                hash,
            }
        } else {
            let (label, focal_length) = value.split_once("=").unwrap();
            let label = label.to_string();
            let hash = hash(&label);

            Self {
                label,
                operation: Operation::Equal,
                focal_length: focal_length.parse().unwrap(),
                hash,
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Operation {
    Minus,
    Equal,
}

impl From<char> for Operation {
    fn from(value: char) -> Self {
        match value {
            '-' => Self::Minus,
            '=' => Self::Equal,
            _ => panic!("invalid char"),
        }
    }
}

fn hash(value: &String) -> usize {
    value.chars().map(|c| c as usize).fold(0, |current, c| {
        let addition = current + c;
        let multiplication = addition * 17;
        multiplication % 256
    })
}

fn create_hashmap(lens_vec: &Vec<Lens>) -> [Vec<Lens>; 256] {
    let mut map: [Vec<Lens>; 256] = vec![vec![]; 256].try_into().unwrap();
    for lens in lens_vec.clone() {
        let hashed = lens.hash;
        match lens.operation {
            Operation::Equal => {
                if let Some(i) = map[hashed].iter().position(|l| l.label == lens.label) {
                    map[hashed][i] = lens;
                } else {
                    map[hashed].push(lens);
                }
            }
            Operation::Minus => {
                if let Some(i) = map[hashed].iter().position(|l| l.label == lens.label) {
                    map[hashed].remove(i);
                }
            }
        }
    }
    map
}

fn part_1(input: &String) -> usize {
    input.split(",").map(String::from).map(|s| hash(&s)).sum()
}

fn part_2(input: &String) -> usize {
    let lens = input.split(",").map(Lens::from).collect();
    let map = create_hashmap(&lens);
    map.iter()
        .enumerate()
        .map(|(i, b)| {
            b.iter()
                .enumerate()
                .map(|(j, l)| (i + 1) * (j + 1) * l.focal_length)
                .sum::<usize>()
        })
        .sum()
}
fn main() {
    let input = aoc::read_input_lines("15")[0].to_string();

    aoc::print_part_1(part_1(&input));
    aoc::print_part_2(part_2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_input() -> String {
        "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_string()
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input()), 1320);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_input()), 145);
    }
}
