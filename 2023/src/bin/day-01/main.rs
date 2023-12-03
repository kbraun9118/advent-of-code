use aoc_2023::{read_input_lines, print_part_1};

fn part_1(lines: Vec<String>) -> u32 {
    lines
        .iter()
        .map(|line| {
            let left = line.chars().find(|c| c.is_numeric());
            let right = line.chars().rev().find(|c| c.is_numeric());
            format!("{}{}", left.unwrap(), right.unwrap())
                .parse::<u32>()
                .unwrap()
        })
        .sum()
}

fn main() {
    let lines = read_input_lines("01");
    print_part_1(part_1(lines));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        assert_eq!(part_1(input), 142);
    }
}
