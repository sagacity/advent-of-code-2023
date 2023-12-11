use std::collections::HashSet;
use itertools::Itertools;

fn shortest_paths(input: &str, expansion_factor: i32) -> Vec<usize> {
    let mut galaxies = vec![];

    let lines = input.lines();
    let mut max_x = 0;
    let mut max_y = 0;
    for line in lines {
        for x in 0..line.len() as i32 {
            if line.chars().nth(x as usize) == Some('#') {
                galaxies.push((x, max_y));
            }
        }
        max_x = line.len() as i32;
        max_y += 1;
    }

    let mut columns_to_expand = vec![];
    let mut rows_to_expand = vec![];

    let mut y = 0;
    while y < max_y {
        if !galaxies.iter().any(|(_, gy)| *gy == y) {
            rows_to_expand.push(y);
        }
        y += 1;
    }
    let mut x = 0;
    while x < max_x {
        if !galaxies.iter().any(|(gx, _)| *gx == x) {
            columns_to_expand.push(x);
        }
        x += 1;
    }

    let mut distances = vec![];
    let mut visited = HashSet::new();
    let ids: Vec<usize> = (0..galaxies.len()).collect();
    for vpair in ids.iter().permutations(2).collect::<Vec<_>>() {
        let pair = (*vpair[0], *vpair[1]);
        if visited.contains(&pair) || visited.contains(&(pair.1, pair.0)) {
            continue;
        }
        visited.insert(pair);

        let (mut ax, mut ay) = galaxies[pair.0];
        let (mut bx, mut by) = galaxies[pair.1];

        ax += expansion_factor * columns_to_expand.iter().filter(|e| **e < ax).collect::<Vec<_>>().len() as i32;
        bx += expansion_factor * columns_to_expand.iter().filter(|e| **e < bx).collect::<Vec<_>>().len() as i32;
        ay += expansion_factor * rows_to_expand.iter().filter(|e| **e < ay).collect::<Vec<_>>().len() as i32;
        by += expansion_factor * rows_to_expand.iter().filter(|e| **e < by).collect::<Vec<_>>().len() as i32;

        let dist = (bx - ax).abs() + (by - ay).abs();
        distances.push(dist as usize);
    }

    distances
}

fn main() {
    let input = include_str!("day11.txt");
    println!("distance sum: {}", shortest_paths(input, 1).into_iter().sum::<usize>());
    println!("distance sum: {}", shortest_paths(input, 999999).into_iter().sum::<usize>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;
        assert_eq!(shortest_paths(input, 1).into_iter().sum::<usize>(), 374);
        assert_eq!(shortest_paths(input, 9).into_iter().sum::<usize>(), 1030);
        assert_eq!(shortest_paths(input, 99).into_iter().sum::<usize>(), 8410);
    }
}