use itertools::Itertools;

fn find_reflections(input: &str) -> Vec<usize> {
    let mut results = vec![];
    let mut pat = vec![];
    for line in input.lines() {
        if !line.is_empty() {
            pat.push(line);
        } else {
            results.push(find_smudged_reflection(&mut pat));
            pat.clear();
        }
    }
    results.push(find_smudged_reflection(&mut pat));
    results
}

fn find_smudged_reflection(pat: &mut Vec<&str>) -> usize {
    let original_reflection = find_reflection(pat, None).unwrap();
    for y in 0..pat.len() {
        let line = &pat[y];
        for x in 0..line.len() {
            let orig = line.chars().nth(x).unwrap();
            let replacement = if orig == '.' {
                '#'
            } else {
                '.'
            };

            let mut pat_copy = pat.clone().iter().map(|l| l.to_string()).collect_vec();
            pat_copy[y].replace_range(x..x + 1, &replacement.to_string());
            let mut foo = vec![];
            for l in &pat_copy {
                foo.push(l.as_str());
            }

            /*for line in &foo {
                println!("{}", line);
            }
            println!("-----^^^ smudged, original vvvv");*/

            if let Some(refl) = find_reflection(&foo, Some(original_reflection)) {
                return refl;
            }

        }
    }
    panic!()
}

fn find_reflection(pat: &Vec<&str>, ignore: Option<usize>) -> Option<usize> {
    // Vertical
    for y in 0..pat.len() {
        let mut is_mirror = true;

        let mut compared_at_least_once = false;
        let mut a_y = y as i64;
        let mut b_y = y + 1;
        loop {
            let a = if a_y >= 0 { pat.get(a_y as usize) } else { None };
            let b = pat.get(b_y);
            if let (Some(a), Some(b)) = (a, b) {
                if a != b {
                    is_mirror = false;
                }
                compared_at_least_once = true;
            }

            if a_y > 0 || b_y < pat.len() {
                a_y -= 1;
                b_y += 1;
            } else {
                break;
            }
        }
        if is_mirror && compared_at_least_once {
            let val = Some((y + 1) * 100);
            if val != ignore {
                return val;
            }
        }
    }

    let get_column = |x: usize| -> Option<String> {
        let mut result = "".to_string();
        for y in 0..pat.len() {
            if let Some(char) = pat[y].chars().nth(x) {
                result += &char.to_string();
            } else {
                return None;
            }
        }
        Some(result)
    };

    // Horizontal
    let line_len = pat[0].len();
    for x in 0..line_len {
        let mut is_mirror = true;

        let mut compared_at_least_once = false;
        let mut a_x = x as i64;
        let mut b_x = x + 1;
        loop {
            let a = if a_x >= 0 { get_column(a_x as usize) } else { None };
            let b = get_column(b_x);
            match (a, b) {
                (Some(a), Some(b)) => {
                    if a != b {
                        is_mirror = false;
                    }
                    compared_at_least_once = true;
                },
                _ => ()
            }

            if a_x > 0 || b_x < line_len {
                a_x -= 1;
                b_x += 1;
            } else {
                break;
            }
        }
        if is_mirror && compared_at_least_once {
            let val = Some(x + 1);
            if val != ignore {
                return val;
            }
        }
    }

    None
}

fn main() {
    let input = include_str!("day13.txt");
    println!("sum: {}", find_reflections(input).into_iter().sum::<usize>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x() {
        let input = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."#;
        assert_eq!(find_reflection(&input.lines().collect(), None), Some(5));
    }

    #[test]
    fn test_x_smudged() {
        let input = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."#;
        assert_eq!(find_smudged_reflection(&mut input.lines().collect()), 300);
    }

    #[test]
    fn test_y() {
        let input = r#"#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;
        assert_eq!(find_reflection(&input.lines().collect(), None), Some(400));
    }

    #[test]
    fn test_y_smudged() {
        let input = r#"#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;
        assert_eq!(find_smudged_reflection(&mut input.lines().collect()), 100);
    }

    #[test]
    fn borked() {
        let input = r#"#.##..#..######..
....##.##..##..##
###...##..#..#..#
###...##..#..#..#
....##.##..##..##
#.##..#..######..
.#..###.###..###.
####...###...####
######..########.
.#..#....#.##.#..
##...###.#....#.#"#;
        assert_eq!(find_smudged_reflection(&mut input.lines().collect()), 12);
    }

    #[test]
    fn example1() {
        let input = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;
        assert_eq!(find_reflections(input).into_iter().sum::<usize>(), 405);
    }
}