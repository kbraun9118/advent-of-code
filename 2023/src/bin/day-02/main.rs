use std::collections::HashMap;

#[derive(Debug)]
struct Game {
    id: u32,
    reveals: Vec<Reveal>,
}

impl Game {
    fn valid(&self, red: u32, green: u32, blue: u32) -> bool {
        !self
            .reveals
            .iter()
            .any(|r| r.red > red || r.green > green || r.blue > blue)
    }

    fn power(&self) -> u32 {
        let (r, g, b) = self.reveals.iter().fold((0, 0, 0), |(r, g, b), reveal| {
            (r.max(reveal.red), g.max(reveal.green), b.max(reveal.blue))
        });
        r * g * b
    }
}

impl From<String> for Game {
    fn from(value: String) -> Self {
        let (game, reveals) = value.split_once(":").unwrap();
        let reveals = reveals.split(";");
        let (_, id) = game.split_once(" ").unwrap();
        Game {
            id: id.parse().unwrap(),
            reveals: reveals.map(|s| s.to_string()).map(|s| s.into()).collect(),
        }
    }
}

#[derive(Debug)]
struct Reveal {
    red: u32,
    blue: u32,
    green: u32,
}

impl From<String> for Reveal {
    fn from(value: String) -> Self {
        let mut values = value
            .split(",")
            .map(|v| {
                let (num, color) = v.trim().split_once(" ").unwrap();
                (color, num.parse::<u32>().unwrap())
            })
            .collect::<HashMap<_, _>>();
        Self {
            red: values.remove("red").unwrap_or_default(),
            blue: values.remove("blue").unwrap_or_default(),
            green: values.remove("green").unwrap_or_default(),
        }
    }
}

fn part_1(games: &Vec<Game>) -> u32 {
    games
        .iter()
        .filter(|g| g.valid(12, 13, 14))
        .map(|g| g.id)
        .sum()
}

fn part_2(games: &Vec<Game>) -> u32 {
    games.iter().map(Game::power).sum()
}

fn main() {
    let games = aoc::read_input_lines("02")
        .into_iter()
        .map(Game::from)
        .collect::<Vec<_>>();

    aoc::print_part_1(part_1(&games));
    aoc::print_part_2(part_2(&games));
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> Vec<Game> {
        r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#
            .lines()
            .map(String::from)
            .map(Game::from)
            .collect()
    }

    #[test]
    fn test_part_1() {
        let games = test_input();

        assert_eq!(part_1(&games), 8);
    }

    #[test]
    fn test_part_2() {
        let games = test_input();

        assert_eq!(part_2(&games), 2286);
    }
}
