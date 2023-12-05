use std::collections::HashMap;

struct Map {
    data: String,
    width: i32,
    height: i32,
}

impl Map {
    fn new(input: &str) -> Self {
        let width = input.lines().nth(0).unwrap().len() as i32;
        let height = input.lines().collect::<Vec<_>>().len() as i32;

        Self {
            data: input.to_string(),
            width,
            height,
        }
    }

    pub fn get(&self, x: i32, y: i32) -> Option<char> {
        self.data.lines().nth(y as usize).map(|line| line.chars().nth(x as usize)).flatten()
    }

    pub fn is_part(&self, x: i32, y: i32) -> bool {
        match self.get(x, y) {
            Some('0'..='9') => false,
            Some('.') => false,
            None => false,
            _ => true,
        }
    }

    pub fn update_gear_pos(&self, x: i32, y: i32, gear_pos: &mut Option<(i32, i32)>) -> bool {
        match self.get(x, y) {
            Some('*') => {
                *gear_pos = Some((x, y));
                true
            },
            _ => false,
        }
    }
}

fn calc_part_numbers(input: &str) -> Vec<u32> {
    let mut parts = vec![];
    let map = Map::new(input);

    let mut cur_part = None;
    let mut cur_part_valid = false;
    for y in 0..map.height {
        for x in 0..map.width {
            match map.get(x, y) {
                number @ Some('0'..='9') => {
                    let number = number.unwrap().to_digit(10).unwrap();
                    cur_part = match cur_part {
                        Some(val) => Some((val * 10) + number),
                        None => Some(number)
                    };

                    cur_part_valid |= map.is_part(x - 1, y - 1);
                    cur_part_valid |= map.is_part(x + 0, y - 1);
                    cur_part_valid |= map.is_part(x + 1, y - 1);
                    cur_part_valid |= map.is_part(x - 1, y);
                    cur_part_valid |= map.is_part(x + 1, y);
                    cur_part_valid |= map.is_part(x - 1, y + 1);
                    cur_part_valid |= map.is_part(x + 0, y + 1);
                    cur_part_valid |= map.is_part(x + 1, y + 1);
                },
                _ => {
                    if cur_part_valid {
                    if let Some(part) = cur_part {
                        parts.push(part);
                    }
                        }
                    cur_part = None;
                    cur_part_valid = false;
                }
            }
        }
    }

    parts
}

fn calc_gear_ratios(input: &str) -> Vec<u32> {
    let mut related_gears: HashMap<(i32, i32), Vec<u32>> = HashMap::new();
    let map = Map::new(input);

    let mut cur_part = None;
    let mut gear_pos = None;
    for y in 0..map.height {
        for x in 0..map.width {
            match map.get(x, y) {
                number @ Some('0'..='9') => {
                    let number = number.unwrap().to_digit(10).unwrap();
                    cur_part = match cur_part {
                        Some(val) => Some((val * 10) + number),
                        None => Some(number)
                    };

                    map.update_gear_pos(x - 1, y - 1, &mut gear_pos);
                    map.update_gear_pos(x + 0, y - 1, &mut gear_pos);
                    map.update_gear_pos(x + 1, y - 1, &mut gear_pos);
                    map.update_gear_pos(x - 1, y, &mut gear_pos);
                    map.update_gear_pos(x + 1, y, &mut gear_pos);
                    map.update_gear_pos(x - 1, y + 1, &mut gear_pos);
                    map.update_gear_pos(x + 0, y + 1, &mut gear_pos);
                    map.update_gear_pos(x + 1, y + 1, &mut gear_pos);
                },
                _ => {
                    if let Some(gear_pos) = gear_pos {
                        if let Some(part) = cur_part {
                            related_gears.entry(gear_pos).or_insert_with(Vec::new).push(part);
                        }
                    }
                    cur_part = None;
                    gear_pos = None;
                }
            }
        }
    }

    related_gears.values()
        .filter(|related| related.len() >= 2)
        .map(|related| related.iter().product::<u32>()).collect()
}

fn main() {
    let input = include_str!("day3.txt");
    println!("Sum: {}", calc_part_numbers(input).iter().sum::<u32>());
    println!("Gear ratios: {}", calc_gear_ratios(input).iter().sum::<u32>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        assert_eq!(calc_part_numbers(input).iter().sum::<u32>(), 4361);
        assert_eq!(calc_gear_ratios(input).iter().sum::<u32>(), 467835);
    }
}
