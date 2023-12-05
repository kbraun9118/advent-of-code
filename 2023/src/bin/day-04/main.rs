use std::collections::HashSet;

#[derive(Debug)]
struct Card {
    winning_numbers: HashSet<u32>,
    player_numbers: HashSet<u32>,
}

impl From<String> for Card {
    fn from(value: String) -> Self {
        let (_, numbers) = value.split_once(":").unwrap();
        let (winning, player) = numbers.split_once("|").unwrap();

        Self {
            winning_numbers: winning
                .trim()
                .split(" ")
                .filter_map(|s| s.trim().parse().ok())
                .collect(),
            player_numbers: player
                .trim()
                .split(" ")
                .filter_map(|s| s.trim().parse().ok())
                .collect(),
        }
    }
}

fn part_1(cards: &Vec<Card>) -> u32 {
    cards
        .iter()
        .map(|c| c.winning_numbers.intersection(&c.player_numbers))
        .filter(|i| i.clone().count() != 0)
        .map(|i| 2u32.pow(i.count() as u32 - 1))
        .sum()
}

fn part_2(cards: &Vec<Card>) -> u32 {
    let mut copies = vec![0; cards.len()];
    let original_winners = cards
        .iter()
        .enumerate()
        .map(|(i, c)| (i, c.winning_numbers.intersection(&c.player_numbers).count()));
    let mut sum = 0;

    for (i, card) in original_winners {
        if card > 0 {
            for j in i + 1..=i + card {
                copies[j] += 1 + copies[i];
            }
            println!("{copies:#?}");
        }

        sum += 1 + copies[i] as u32
    }

    sum as u32
}

fn main() {
    let cards = aoc::read_input_lines("04")
        .into_iter()
        .map(Card::from)
        .collect::<Vec<_>>();

    aoc::print_part_1(part_1(&cards));
    aoc::print_part_2(part_2(&cards));
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> Vec<Card> {
        r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#
            .lines()
            .map(String::from)
            .map(Card::from)
            .collect()
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&test_input()), 13)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&test_input()), 30)
    }
}
