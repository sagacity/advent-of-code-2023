use std::collections::HashMap;
use rayon::prelude::*;

fn calc_combinations(input: &str, unfold: bool) -> Vec<usize> {
    input.par_lines()
        .map(|line| {
            let split = line.split(' ').collect::<Vec<_>>();
            let springs = split[0].to_string();
            let mut expected = split[1].split(',').map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();

            let springs = if unfold {
                let s = springs.to_string();
                s.clone() + "?" + &s.clone() + "?" + &s.clone() + "?" + &s.clone() + "?" + &s.clone()
            } else {
                springs
            };

            if unfold {
                expected = vec![expected.clone(), expected.clone(), expected.clone(), expected.clone(), expected.clone()].into_iter().flatten().collect::<Vec<_>>();
            }

            calc(&mut HashMap::new(), &springs, &expected, 0, 0,0)
        })
        .collect()
}

fn calc(cache: &mut HashMap<(usize, usize, usize), usize>, springs: &str, expected: &[usize], springs_pos: usize, cur_group: usize, inside_group_length: usize) -> usize {
    if let Some(result) = cache.get(&(springs_pos, cur_group, inside_group_length)) {
        return *result;
    }

    if springs_pos == springs.len() {
        let result = if (inside_group_length == 0 && cur_group == expected.len()) || (cur_group == expected.len() - 1 && expected[cur_group] == inside_group_length) {
            1
        } else {
            0
        };
        cache.insert((springs_pos, cur_group, inside_group_length), result);
        return result;
    }

    let mut result = 0;

    if springs.chars().nth(springs_pos) == Some('#') || springs.chars().nth(springs_pos) == Some('?') {
        result += calc(cache, springs, expected, springs_pos + 1, cur_group, inside_group_length + 1);
    }

    if springs.chars().nth(springs_pos) == Some('.') || springs.chars().nth(springs_pos) == Some('?') {
        if inside_group_length == 0 {
            result += calc(cache, springs, expected, springs_pos + 1, cur_group, 0);
        } else if cur_group < expected.len() && expected[cur_group] == inside_group_length {
            result += calc(cache, springs, expected, springs_pos + 1, cur_group + 1, 0);
        }
    }

    cache.insert((springs_pos, cur_group, inside_group_length), result);
    result
}

fn main() {
    let input = include_str!("day12.txt");
    println!("sum: {}", calc_combinations(input, false).into_iter().sum::<usize>());
    println!("sum: {}", calc_combinations(input, true).into_iter().sum::<usize>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_basic() {
        let input = "?.# 1,1";
        assert_eq!(calc_combinations(input, false).into_iter().sum::<usize>(), 1);
        assert_eq!(calc_combinations(input, true).into_iter().sum::<usize>(), 1);
    }

    #[test]
    fn example_simple() {
        let input = "???.### 1,1,3";
        assert_eq!(calc_combinations(input, false).into_iter().sum::<usize>(), 1);
        assert_eq!(calc_combinations(input, true).into_iter().sum::<usize>(), 1);
    }

    #[test]
    fn example_simple2() {
        let input = "?###???????? 3,2,1";
        assert_eq!(calc_combinations(input, false).into_iter().sum::<usize>(), 10);
        assert_eq!(calc_combinations(input, true).into_iter().sum::<usize>(), 506250);
    }

    #[test]
    fn example_simple3() {
        let input = "????.######..#####. 1,6,5";
        assert_eq!(calc_combinations(input, true).into_iter().sum::<usize>(), 2500);
    }

    #[test]
    fn example1() {
        let input = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;
        assert_eq!(calc_combinations(input, false).into_iter().sum::<usize>(), 21);
        assert_eq!(calc_combinations(input, true).into_iter().sum::<usize>(), 525152);
    }
}