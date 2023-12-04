use std::collections::BTreeMap;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, multispace1};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded, tuple};

type IResult<'a, T> = nom::IResult<&'a str, T>;

#[derive(Clone, Debug)]
struct Card {
    id: u32,
    winning: Vec<u32>,
    have: Vec<u32>
}

impl Card {
    pub fn num_matching(&self) -> u32 {
        self.have.iter().filter(|have| self.winning.contains(have)).collect::<Vec<_>>().len() as u32
    }

    pub fn score(&self) -> usize {
        let mut score = 0;
        for have in &self.have {
            if self.winning.contains(&have) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }
        score
    }
}

fn parse_card(input: &str) -> IResult<Card> {
    map(tuple((
                  delimited(tuple((tag("Card"), multispace1)), digit1, tuple((tag(":"), multispace1))),
                  separated_list1(multispace1, digit1),
                  preceded(tuple((tag(" |"), multispace1)), separated_list1(multispace1, digit1)),
              )), |(id, winning, have)| {
        Card {
            id: u32::from_str_radix(id, 10).unwrap(),
            winning: winning.into_iter().map(|val| u32::from_str_radix(val, 10).unwrap()).collect(),
            have: have.into_iter().map(|val| u32::from_str_radix(val, 10).unwrap()).collect(),
        }
    })(input)
}

fn parse_cards(input: &str) -> Vec<Card> {
    input.lines()
        .map(|line| {
            let (_, card) = parse_card(line).unwrap();
            card
        })
        .collect()
}

fn get_num_winning_cards(cards: &[Card]) -> usize {
    let cards = cards.into_iter().map(|card| (card.id, card.clone())).collect::<BTreeMap<_, _>>();
    let mut won_ids: Vec<u32> = vec![];
    for (card_id, _) in &cards {
        won_ids.push(*card_id);
    }
    let mut idx = 0;
    while idx < won_ids.len() {
        let card_id = *won_ids.get(idx).unwrap();
        let card = cards.get(&card_id).unwrap();
        let num_matching = card.num_matching();
        for i in 0..num_matching {
            won_ids.push(card_id + i + 1);
        }
        idx += 1;
    }
    won_ids.len()
}

fn main() {
    let cards = parse_cards(include_str!("day4.txt"));
    println!("Score sum: {}", cards.iter().map(|card| card.score()).sum::<usize>());
    println!("Total scratchcards: {}", get_num_winning_cards(&cards));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
        let cards = parse_cards(input);
        assert_eq!(cards.iter().map(|card| card.score()).sum::<usize>(), 13);
        assert_eq!(get_num_winning_cards(&cards), 30);
    }
}