#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn possible_wins(&self) -> usize {
        let mut wins = 0;
        for time_held in 1..self.time {
            let time_left = self.time - time_held;
            if time_left * time_held > self.distance {
                wins += 1;
            }
        }

        wins
    }
}

impl From<(&str, &str)> for Race {
    fn from((time, distance): (&str, &str)) -> Self {
        Race {
            distance: distance.parse().unwrap(),
            time: time.parse().unwrap(),
        }
    }
}

fn part_1(input: &Vec<String>) -> usize {
    let split = input
        .iter()
        .map(|s| s.split_once(":").unwrap().1)
        .collect::<Vec<_>>();

    let races = split[0]
        .split_whitespace()
        .zip(split[1].split_whitespace())
        .map(Race::from)
        .collect::<Vec<_>>();

    races
        .iter()
        .map(Race::possible_wins)
        .fold(1, |acc, next| acc * next)
}

fn part_2(input: &Vec<String>) -> usize {
    let split = input
        .iter()
        .map(|s| s.split_once(":").unwrap().1)
        .map(|s| s.replace(" ", ""))
        .collect::<Vec<_>>();

    let race = Race::from((split[0].trim(), split[1].trim()));

    race.possible_wins()
}

fn main() {
    let input = aoc::read_input_lines("06");

    aoc::print_part_1(part_1(&input));
    aoc::print_part_2(part_2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_input() -> Vec<String> {
        r#"Time:      7  15   30
Distance:  9  40  200"#
            .lines()
            .map(String::from)
            .collect()
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input()), 288);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_input()), 71503);
    }
}
