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

            calc(&springs, &expected)
        })
        .collect()
}

fn calc(springs: &str, expected: &[usize]) -> usize {
    let mut to_check_vec = vec!["".to_string()];
    let mut valid = 0;

    while let Some(to_check) = to_check_vec.pop() {
        let actual = get_actual_groups(&to_check);
        if to_check.len() == springs.len() {
            if actual == expected {
                valid += 1;
            }
            continue;
        }
        if actual.len() > expected.len() || actual.iter().zip(expected.iter()).any(|(a, e)| *a > *e) {
            continue;
        }
        if actual.len() > 0 && actual[0..actual.len() - 1].iter().zip(expected[0..expected.len() - 1].iter()).any(|(a, e)| *a != *e) {
            continue;
        }

        if let Some(ch) = springs.chars().nth(to_check.len()) {
            match ch {
                '#' => to_check_vec.push(to_check.to_string() + "#"),
                '.' => to_check_vec.push(to_check.to_string() + "."),
                '?' => {
                    to_check_vec.push(to_check.to_string() + "#");
                    to_check_vec.push(to_check.to_string() + ".");
                },
                _ => panic!()
            }
        }
    }

    valid
}

fn get_actual_groups(springs: &str) -> Vec<usize> {
    let mut actual = vec![];
    let mut cur_total = 0;
    for c in springs.chars() {
        if c == '#' {
            cur_total += 1;
        } else if cur_total > 0 {
            actual.push(cur_total);
            cur_total = 0;
        }
    }
    if cur_total > 0 {
        actual.push(cur_total);
    }
    actual
}

fn is_valid(line: &str) -> bool {
    let split = line.split(' ').collect::<Vec<_>>();
    let springs = split[0];
    let expected = split[1].split(',').map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
    expected == get_actual_groups(springs)
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
    fn check_is_valid() {
        let input = r#"#.#.### 1,1,3
.#...#....###. 1,1,3
.#.###.#.###### 1,3,1,6
####.#...#... 4,1,1
#....######..#####. 1,6,5
.###.##....# 3,2,1"#;
        for line in input.lines() {
            assert!(is_valid(line));
        }
    }

    #[test]
    fn example_simple() {
        let input = "???.### 1,1,3";
        assert_eq!(calc_combinations(input, false).into_iter().sum::<usize>(), 1);
        assert_eq!(calc_combinations(input, true).into_iter().sum::<usize>(), 1);
    }

    #[test]
    fn example_simple2() {
        let input = "###???????? 3,2,1";
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