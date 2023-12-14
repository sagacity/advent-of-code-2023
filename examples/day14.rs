use std::collections::HashMap;
use itertools::Itertools;

fn calc_load(input: &str) -> usize {
    let lines = input.lines().collect_vec();
    let mut rocks = HashMap::new();
    let max_y = lines.len() as i32;
    let mut max_x = 0;
    for y in 0..lines.len() {
        let line = lines[y];
        max_x = line.len() as i32;
        for x in 0..line.len() {
            match line.chars().nth(x) {
                ch @ Some('#') | ch @ Some('O') => { rocks.insert((x as i32, y as i32), ch.unwrap()); }
                _ => {}
            }
        }
    }

    let mut jumped = false;
    let mut history = vec![];
    let mut cycle = 0;
    while cycle < 1000000000 {
        for dir in vec![(0, -1), (-1, 0), (0, 1), (1, 0)] {
            loop {
                let mut new_rocks = HashMap::new();
                for ((x, y), ch) in &rocks {
                    let new_pos = if *ch == 'O' && *x + dir.0 >= 0 && *y + dir.1 >= 0 && *x + dir.0 < max_x && *y + dir.1 < max_y && !rocks.contains_key(&(*x + dir.0, *y + dir.1)) {
                        (*x + dir.0, *y + dir.1)
                    } else {
                        (*x, *y)
                    };
                    new_rocks.insert(new_pos, *ch);
                }
                if rocks == new_rocks {
                    break;
                }
                rocks = new_rocks;
            }
        }

        /*println!("After cycle {}", cycle + 1);
        for y in 0..lines.len() {
            let line = lines[y];
            for x in 0..line.len() {
                if let Some(ch) = rocks.get(&(x as i32, y as i32)) {
                    print!("{}", ch);
                } else {
                    print!(".");
                }
            }
            println!();
        }*/

        for i in 0..history.len() {
            if history[i] == rocks && !jumped {
                jumped = true;
                println!("{} equals previous cycle: {}", cycle, i);
                let oc = cycle - i;
                while cycle < 1000000000 {
                    cycle += oc;
                }
                cycle -= oc;
                println!("Jumped to: {cycle}");
            }
        }
        history.push(rocks.clone());
        cycle += 1;
    }

    /*for y in 0..lines.len() {
        let line = lines[y];
        for x in 0..line.len() {
            if let Some(ch) = rocks.get(&(x as i32, y as i32)) {
                print!("{}", ch);
            } else {
                print!(".");
            }
        }
        println!();
    }*/

    let max_y = lines.len();

    let mut total = 0;
    for ((_, y), ch) in &rocks {
        if *ch == 'O' {
            total += max_y as i32 - *y;
        }
    }

    total as usize
}

fn main() {
    let input = include_str!("day14.txt");
    println!("load: {}", calc_load(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;
        //assert_eq!(calc_load(input), 136);
        assert_eq!(calc_load(input), 64);
    }
}