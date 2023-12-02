use std::collections::HashMap;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, crlf, digit1, multispace0};
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::sequence::{delimited, pair, tuple};

type IResult<'a, T> = nom::IResult<&'a str, T>;

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    grabs: Vec<Grab>
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Color {
    Red, Green, Blue
}

#[derive(Debug, PartialEq)]
struct Grab {
    amounts: Vec<CubeAmount>
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct CubeAmount {
    color: Color,
    amount: usize,
}

impl CubeAmount {
    pub fn new(color: Color, amount: usize) -> Self {
        Self {
            color,
            amount
        }
    }
}

impl<'a> From<&'a str> for Color {
    fn from(value: &'a str) -> Self {
        match value {
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            _ => panic!()
        }
    }
}

fn parse_color(input: &str) -> IResult<Color> {
    map(alpha1, Color::from)(input)
}

fn parse_grab_amount(input: &str) -> IResult<CubeAmount> {
    map(tuple((
        map_res(digit1, str::parse),
        tag(" "),
        parse_color,
    )), |(amount, _, color)| {
        CubeAmount {
            color,
            amount
        }})(input)
}

fn parse_grab_amounts(input: &str) -> IResult<Vec<CubeAmount>> {
    separated_list1(tag(", "), parse_grab_amount)(input)
}

fn parse_grabs(input: &str) -> IResult<Vec<Grab>> {
    separated_list1(pair(tag(";"), multispace0), map(parse_grab_amounts, |amounts| {
        Grab {
            amounts
        }
    }))(input)
}

fn parse_game(input: &str) -> IResult<Game> {
    map(tuple((delimited(tag("Game "), map_res(digit1, str::parse), tag(": ")), parse_grabs)), |(id, grabs)| Game {
        id,
        grabs,
    })(input)
}

fn parse(input: &str) -> IResult<Vec<Game>> {
    separated_list1(alt((crlf, tag("\n"))), parse_game)(input)
}

fn valid_games(games: &[Game], available_cubes: Vec<CubeAmount>) -> Vec<&Game> {
    games.iter()
        .filter(|game| {
            game.grabs.iter().all(|grab| {
                let mut totals = HashMap::new();
                for amount in &grab.amounts {
                    totals.insert(amount.color, amount.amount);
                }

                for available in &available_cubes {
                    let total_for_cube = totals.get(&available.color).unwrap_or(&0);
                    if *total_for_cube > available.amount {
                        return false;
                    }
                }

                true
            })
        })
        .collect()
}

fn calc_powers(games: &[Game]) -> Vec<usize> {
    games.iter()
        .map(|game| {
            let mut amounts = HashMap::new();

            for grab in &game.grabs {
                for grab_amount in &grab.amounts {
                    if amounts.get(&grab_amount.color).unwrap_or(&0) < &grab_amount.amount {
                        amounts.insert(grab_amount.color, grab_amount.amount);
                    }
                }
            }

            amounts.values().product()
        })
        .collect()
}

fn main() {
    let input = include_str!("day2.txt");
    let (unparsed, parsed) = parse(input).unwrap();
    assert_eq!(unparsed, "");
    println!("Combined ID: {}", valid_games(&parsed, vec![
        CubeAmount::new(Color::Red, 12),
        CubeAmount::new(Color::Green, 13),
        CubeAmount::new(Color::Blue, 14),
    ]).into_iter().map(|game| game.id).sum::<u32>());

    println!("Total cubed: {}", calc_powers(&parsed).iter().sum::<usize>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        let (unparsed, parsed) = parse(input).unwrap();
        assert_eq!(unparsed, "");
        assert_eq!(parsed.len(), 5);
        assert_eq!(parsed[1].id, 2);
        assert_eq!(parsed[1].grabs.len(), 3);
        assert_eq!(parsed[1].grabs[1].amounts, vec![
            CubeAmount::new(Color::Green, 3),
            CubeAmount::new(Color::Blue, 4),
            CubeAmount::new(Color::Red, 1),
        ]);
        assert_eq!(valid_games(&parsed, vec![
            CubeAmount::new(Color::Red, 12),
            CubeAmount::new(Color::Green, 13),
            CubeAmount::new(Color::Blue, 14),
        ]).into_iter().map(|game| game.id).sum::<u32>(), 8);

        assert_eq!(calc_powers(&parsed), vec![
            48, 12, 1560, 630, 36
        ]);
        assert_eq!(calc_powers(&parsed).iter().sum::<usize>(), 2286);
    }
}