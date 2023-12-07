use std::cmp::Ordering;
use itertools::Itertools;

#[derive(Debug, PartialEq)]
struct Hand {
    hand: [Card; 5]
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        Self { hand: value.chars().map(Card::from).collect::<Vec<_>>().try_into().unwrap() }
    }
}

impl Hand {
    fn ty(&self) -> HandType {
        let num_jokers = self.hand.iter().filter(|c| **c == Card::J).count();
        let non_jokers = std::iter::repeat(2..=13).take(num_jokers).multi_cartesian_product().collect::<Vec<_>>();
        let cards = self.hand.clone();

        let mut group_maps = vec![];
        group_maps.push(cards.clone().into_iter().into_group_map_by(|card| card.clone()));
        for perm_idx in 0..non_jokers.len() {
            let mut nj_idx = 0;
            let mut cards = cards.clone();
            for i in 0..5 {
                if cards[i] == Card::J {
                    cards[i] = match non_jokers[perm_idx][nj_idx] {
                        2 => Card::Two,
                        3 => Card::Three,
                        4 => Card::Four,
                        5 => Card::Five,
                        6 => Card::Six,
                        7 => Card::Seven,
                        8 => Card::Eight,
                        9 => Card::Nine,
                        10 => Card::T,
                        11 => Card::Q,
                        12 => Card::K,
                        13 => Card::A,
                        _ => panic!()
                    };
                    nj_idx += 1;
                }
            }
            //println!("Testing: {:?}", cards);
            group_maps.push(cards.clone().into_iter().into_group_map_by(|card| card.clone()));
        }

        for group_map in &group_maps {
            if group_map.iter().any(|(_, group)| group.len() == 5) {
                return HandType::FiveOfAKind;
            }
        }
        for group_map in &group_maps {
            if group_map.iter().any(|(_, group)| group.len() == 4) {
                return HandType::FourOfAKind;
            }
        }
        for group_map in &group_maps {
            if group_map.iter().any(|(_, group)| group.len() == 3) && group_map.iter().any(|(_, group)| group.len() == 2) {
                return HandType::FullHouse;
            }
        }
        for group_map in &group_maps {
            if group_map.iter().any(|(_, group)| group.len() == 3) {
                return HandType::ThreeOfAKind;
            }
        }
        for group_map in &group_maps {
            if group_map.iter().filter(|(_, group)| group.len() == 2).collect::<Vec<_>>().len() == 2 {
                return HandType::TwoPair;
            }
        }
        for group_map in &group_maps {
            if group_map.iter().filter(|(_, group)| group.len() == 2).collect::<Vec<_>>().len() == 1 {
                return HandType::OnePair;
            }
        }

        for group_map in &group_maps {
            let distinct = group_map.iter().all(|(_, group)| group.len() == 1);
            if distinct {
                return HandType::HighCard;
            }
        }

        HandType::Nothing
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
enum HandType {
    Nothing,
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.ty() > other.ty() {
            return Some(Ordering::Greater);
        }
        if self.ty() < other.ty() {
            return Some(Ordering::Less);
        }

        for i in 0..5 {
            if self.hand[i] > other.hand[i] {
                return Some(Ordering::Greater);
            }
            if self.hand[i] < other.hand[i] {
                return Some(Ordering::Less);
            }
        }

        Some(Ordering::Equal)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => panic!()
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.score().partial_cmp(&other.score())
    }
}

impl Card {
    fn score(&self) -> usize {
        match self {
            Card::A => 14,
            Card::K => 13,
            Card::Q => 12,
            //Card::J => 11,
            Card::T => 10,
            Card::Nine => 9,
            Card::Eight => 8,
            Card::Seven => 7,
            Card::Six => 6,
            Card::Five => 5,
            Card::Four => 4,
            Card::Three => 3,
            Card::Two => 2,
            Card::J => 1,
        }
    }
}

fn parse_input(input: &str) -> Vec<(Hand, usize)> {
    input.lines()
        .map(|line| {
            let split = line.split(' ').collect::<Vec<_>>();
            (Hand::from(split[0]), split[1].parse::<usize>().unwrap())
        })
        .collect()
}

fn main() {
    let input = include_str!("day7.txt");
    let mut card_ranks = parse_input(input);
    card_ranks.sort_by(|(a, _), (b, _ )| a.partial_cmp(b).unwrap());
    println!("score: {}", card_ranks.iter().enumerate().map(|(idx, (_, b))| b * (idx + 1)).sum::<usize>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn algo() {
        assert_eq!(Hand::from("33332").ty(), HandType::FourOfAKind);
        assert_eq!(Hand::from("2AAAA").ty(), HandType::FourOfAKind);
        assert_eq!(Hand::from("33332") > Hand::from("2AAAA"), true);

        assert_eq!(Hand::from("77888").ty(), HandType::FullHouse);
        assert_eq!(Hand::from("77788").ty(), HandType::FullHouse);
        assert_eq!(Hand::from("77888") > Hand::from("77788"), true);

        assert_eq!(Hand::from("KK677") > Hand::from("KTJJT"), true);
        assert_eq!(Hand::from("KK677") == Hand::from("KK677"), true);
        assert_eq!(Hand::from("KTJJT") < Hand::from("KK677"), true);
        assert_eq!(Hand::from("KTJJT") == Hand::from("KTJJT"), true);
        assert_eq!(Hand::from("32T3K") < Hand::from("KK677"), true);
        assert_eq!(Hand::from("32T3K") < Hand::from("KTJJT"), true);
    }

    #[test]
    fn algo_jokers() {
        assert_eq!(Hand::from("32T3K").ty(), HandType::OnePair);
        assert_eq!(Hand::from("KK677").ty(), HandType::TwoPair);
        assert_eq!(Hand::from("T55J5").ty(), HandType::FourOfAKind);
        assert_eq!(Hand::from("KTJJT").ty(), HandType::FourOfAKind);
        assert_eq!(Hand::from("QQQJA").ty(), HandType::FourOfAKind);
    }

    #[test]
    fn example1() {
        let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
        let mut card_ranks = parse_input(input);
        card_ranks.sort_by(|(a, _), (b, _ )| a.partial_cmp(b).unwrap());
        let score = card_ranks.iter().enumerate().map(|(idx, (_, b))| b * (idx + 1)).sum::<usize>();
        assert_eq!(score, 6440);
    }
}