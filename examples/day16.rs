use std::collections::HashSet;
use itertools::Itertools;

type Vec2 = (i64, i64);

struct Map {
    lines: Vec<String>
}

impl Map {
    pub fn visualise(&self, tiles: &HashSet<(Vec2, Vec2)>) {
        let tiles = tiles.iter().map(|(pos, _)| *pos).collect_vec();
        for y in 0..self.lines.len() {
            for x in 0..self.lines[y].len() {
                if tiles.contains(&(x as i64, y as i64)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    pub fn get(&self, pos: Vec2) -> Option<char> {
        if pos.1 >= 0 && pos.1 < self.lines.len() as i64 {
            let line = &self.lines[pos.1 as usize];
            line.chars().nth(pos.0 as usize)
        } else {
            None
        }
    }
}

fn calc_tiles(input: &str) -> usize {
    let mut tiles = HashSet::new();
    let map = Map {
        lines: input.lines().map(|l| l.to_string()).collect_vec()
    };
    let max_x = map.lines[0].chars().collect_vec().len();

    let mut max = 0;

    let mut posdirs: Vec<(Vec2, Vec2)> = vec![];
    for y in 0..map.lines.len() {
        posdirs.push(((0, y as i64), (1, 0)));
        posdirs.push((((max_x - 1) as i64, y as i64), (-1, 0)));
    }
    for x in 0..max_x {
        posdirs.push(((x as i64, 0), (0, 1)));
        posdirs.push(((x as i64, (map.lines.len() - 1) as i64), (0, -1)));
    }
    for (pos, dir) in posdirs {
        let mut tiles = tiles.clone();
        propagate(pos, dir, &mut tiles, &map);
        //map.visualise(&tiles);

        let tiles = tiles.into_iter().map(|(pos, _)| pos).collect::<HashSet<_>>();
        let res = tiles.into_iter().collect_vec().len();
        if res > max {
            max = res;
        }
    }
    max
}

fn propagate(pos: Vec2, dir: Vec2, tiles: &mut HashSet<(Vec2, Vec2)>, map: &Map) {
    if tiles.contains(&(pos, dir)) {
        return;
    }
    //println!("{:?} {:?} --> {:?}", pos, dir, map.get(pos));

    let new_dirs = match map.get(pos) {
        Some('.') => {
            vec![dir]
        },
        Some('/') => {
            vec![match dir {
                (1, 0) => (0, -1),
                (0, 1) => (-1, 0),
                (-1, 0) => (0, 1),
                (0, -1) => (1, 0),
                _ => panic!()
            }]
        }
        Some('\\') => {
            vec![match dir {
                (1, 0) => (0, 1),
                (0, 1) => (1, 0),
                (-1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                _ => panic!()
            }]
        }
        Some('|') => {
            match dir {
                (1, 0) => vec![(0, -1), (0, 1)],
                (0, 1) => vec![(0, 1)],
                (-1, 0) => vec![(0, 1), (0, -1)],
                (0, -1) => vec![(0, -1)],
                _ => panic!()
            }
        }
        Some('-') => {
            match dir {
                (1, 0) => vec![(1, 0)],
                (0, 1) => vec![(-1, 0), (1, 0)],
                (-1, 0) => vec![(-1, 0)],
                (0, -1) => vec![(-1, 0), (1, 0)],
                _ => panic!()
            }
        }
        None => {
            return;
        },
        _ => panic!()
    };

    tiles.insert((pos, dir));
    for d in new_dirs {
        propagate((pos.0 + d.0, pos.1 + d.1), d, tiles, map);
    }
}

fn main() {
    let input = include_str!("day16.txt");
    println!("lit tiles: {}", calc_tiles(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
        assert_eq!(calc_tiles(input), 51);
    }
}