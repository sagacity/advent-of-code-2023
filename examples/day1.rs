fn calc(input: &str) -> Vec<u32> {
    let mappings = ["~~~~", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    input.lines().into_iter()
        .map(|to_map| {
            let mut line = String::default();
            for idx in 0..to_map.len() {
                for (mapping_idx, mapping) in mappings.iter().enumerate() {
                    if to_map.get(idx..idx + mapping.len()) == Some(mapping) {
                        line.push(char::from_digit(mapping_idx as u32, 10).unwrap());
                        break;
                    }
                }
                line.push(to_map.chars().nth(idx).unwrap());
            }

            let numbers = line.chars().filter_map(|ch| {
                ch.to_digit(10)
            }).collect::<Vec<u32>>();
            let first = numbers.first().unwrap();
            let last = numbers.last().unwrap();
            //println!("{line} --> {first} / {last}");
            first * 10 + last
        })
        .collect()
}

fn main() {
    let results = calc(include_str!("day1.txt"));
    println!("sum: {}", results.into_iter().sum::<u32>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_day1_1() {
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
        let results = calc(input);
        assert_eq!(results.into_iter().sum::<u32>(), 142);
    }

    #[test]
    fn example_day1_2() {
        let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
        let results = calc(input);
        assert_eq!(results.into_iter().sum::<u32>(), 281);
    }
}