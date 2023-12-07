use std::collections::HashMap;

// Ordering of enum definition matters for PartialOrd derivation
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Card {
    Number(u32),
    Jack,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '2'..='9' => Self::Number(value.to_digit(10).unwrap()),
            'T' => Self::Number(10),
            'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => panic!("Invalid character"),
        }
    }
}

impl Card {
    fn special_ordering(&self,other: &Self, is_joker: bool) -> Option<std::cmp::Ordering> {
        if !is_joker {
            self.partial_cmp(other)
        } else {
            use Card::*;
            use std::cmp::Ordering::*;
            Some(match (*self, other) {
                (Jack, Jack) => Equal,
                (Jack, _) => Less,
                (_, Jack) => Greater,
                _ => self.partial_cmp(other).unwrap()
            })
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Hand {
    cards: [Card; 5],
    bet: u32,
    joker: bool,
}

impl Hand {
    fn into_joker(self) -> Self {
        Self {
            joker: true,
            ..self
        }
    }

    fn rank(&self) -> u32 {
        let mut card_occurance = self.cards.iter().fold(HashMap::new(), |mut map, card| {
            map.entry(card.clone())
                .and_modify(|count| *count += 1)
                .or_insert(1);
            map
        });

        if !self.joker {
            if card_occurance.values().any(|c| *c == 5) {
                7
            } else if card_occurance.values().any(|c| *c == 4) {
                6
            } else if card_occurance.values().any(|c| *c == 3)
                && card_occurance.values().any(|c| *c == 2)
            {
                5
            } else if card_occurance.values().any(|c| *c == 3) {
                4
            } else if card_occurance.values().filter(|c| **c == 2).count() == 2 {
                3
            } else if card_occurance.values().any(|c| *c == 2) {
                2
            } else {
                1
            }
        } else {
            let joker_count = card_occurance.remove(&Card::Jack).unwrap_or(0);
            if card_occurance.values().any(|c| *c + joker_count == 5) {
                7
            } else if card_occurance.values().any(|c| *c + joker_count == 4) {
                println!("Four {:?}", self.cards);
                6
            } else if card_occurance.values().any(|c| *c + joker_count == 3)
                && card_occurance.values().any(|c| *c == 2)
            {
                5
            } else if card_occurance.values().any(|c| *c + joker_count == 3) {
                4
            } else if card_occurance.values().filter(|c| **c == 2).count() == 2 {
                3
            } else if card_occurance.values().any(|c| *c + joker_count == 2) {
                2
            } else {
                1
            }
        }
    }
}

impl From<String> for Hand {
    fn from(value: String) -> Self {
        let (cards, bet) = value.split_once(" ").unwrap();
        let cards = cards.chars().map(Card::from).collect::<Vec<_>>();

        Hand {
            cards: cards.try_into().unwrap(),
            bet: bet.parse().unwrap(),
            joker: false,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_rank = self.rank();
        let other_rank = other.rank();

        if self_rank == other_rank {
            self.cards
                .iter()
                .zip(other.cards.iter())
                .find(|(s, o)| s != o)
                .map_or(Some(std::cmp::Ordering::Equal), |(s, o)| s.special_ordering(o, self.joker))
        } else {
            self_rank.partial_cmp(&other_rank)
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn part_1(hands: &Vec<Hand>) -> u32 {
    let mut hands = hands.into_iter().collect::<Vec<_>>();
    hands.sort();

    hands
        .into_iter()
        .enumerate()
        .map(|(i, h)| (i as u32 + 1) * h.bet)
        .sum()
}

fn part_2(hands: &Vec<Hand>) -> u32 {
    let mut hands = hands
        .iter()
        .map(|h| h.into_joker())
        .collect::<Vec<_>>();

    hands.sort();

    hands
        .into_iter()
        .enumerate()
        .map(|(i, h)| (i as u32 + 1) * h.bet)
        .sum()
}

fn main() {
    let hands = aoc::read_input_lines("07")
        .into_iter()
        .map(Hand::from)
        .collect::<Vec<_>>();

    aoc::print_part_1(part_1(&hands));
    aoc::print_part_2(part_2(&hands));
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_input() -> Vec<Hand> {
        r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#
            .lines()
            .map(String::from)
            .map(Hand::from)
            .collect()
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_test_input()), 6440);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&get_test_input()), 5905);
        assert!(false);
    }

    #[test]
    fn test_joker_compare() {
        assert!(Hand::from("JKKK2 0".to_string()).into_joker() < Hand::from("QQQQ2 0".to_string()).into_joker())
    }
}
