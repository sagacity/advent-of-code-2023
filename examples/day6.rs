use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, multispace0, multispace1};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{preceded, tuple};

type IResult<'a, T> = nom::IResult<&'a str, T>;

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

fn parse_line<'a>(prefix: &str, input: &'a str) -> IResult<'a, Vec<usize>> {
    preceded(tuple((tag(prefix), multispace0)), separated_list1(multispace1, map(alphanumeric1, |val| {
        usize::from_str_radix(val, 10).unwrap_or_default()
    })))(input)
}

fn parse_races(input: &str) -> Vec<Race> {
    let mut input = input.lines();
    let (_, time) = parse_line("Time:", input.next().unwrap()).unwrap();
    let (_, distance) = parse_line("Distance:", input.next().unwrap()).unwrap();
    time.into_iter().zip(distance.into_iter()).map(|(time, distance)| Race {
        time, distance
    }).collect()
}

fn get_options(races: &[Race]) -> Vec<usize> {
    let mut options = vec![];
    for race in races {
        let mut valid = 0;
        for button_time in 0..race.time {
            let total_distance = (race.time - button_time) * button_time;
            if total_distance > race.distance {
                valid += 1;
            }
        }
        options.push(valid);
    }
    options
}

fn main() {
    let input = include_str!("day6.txt");
    let races = parse_races(input);
    println!("Options product: {}", get_options(&races).iter().product::<usize>());

    let input = input.replace(" ", "");
    println!("{:?}", input);
    let races = parse_races(&input);
    println!("Options product: {}", get_options(&races).iter().product::<usize>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = r#"Time:      7  15   30
Distance:  9  40  200"#;
        let races = parse_races(input);
        assert_eq!(get_options(&races).iter().product::<usize>(), 288);
    }
}