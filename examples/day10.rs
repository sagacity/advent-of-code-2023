use std::collections::{HashMap};

type Pos = (i32, i32);

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<Tile>>,
    start: Pos,
}

#[derive(Debug)]
struct Tile {
    ty: char,
}

impl Tile {
    pub fn from(ty: char) -> Self {
        Self {
            ty
        }
    }
}

impl Map {
    pub fn from(tiles: Vec<Vec<Tile>>) -> Self {
        let mut start = None;
        for y in 0..tiles.len() {
            let line = &tiles[y];
            for x in 0..line.len() {
                if line[x].ty == 'S' {
                    start = Some((x as i32, y as i32));
                    break;
                }
            }
        }
        let start = start.unwrap();

        Self {
            tiles,
            start,
        }
    }

    pub fn fix_start_tile(&mut self) {
        let w = self.exits((self.start.0 - 1, self.start.1)).contains(&self.start);
        let e = self.exits((self.start.0 + 1, self.start.1)).contains(&self.start);
        let n = self.exits((self.start.0, self.start.1 - 1)).contains(&self.start);
        let s = self.exits((self.start.0, self.start.1 + 1)).contains(&self.start);
        let tile = match(w, e, n, s) {
            (true, true, false, false) => '-',
            (true, false, true, false) => 'J',
            (true, false, false, true) => '7',
            (false, true, true, false) => 'L',
            (false, false, true, true) => '|',
            (false, true, false, true) => 'F',
            _ => panic!()
        };
        self.tiles[self.start.1 as usize][self.start.0 as usize] = Tile { ty: tile };
    }

    pub fn calc_distances(&self) -> HashMap<Pos, usize> {
        let pos = self.start;
        let mut distances = HashMap::new();

        let mut to_visit = vec![(pos, 0)];

        while let Some((pos, cur_distance)) = to_visit.pop() {
            distances.insert(pos, cur_distance);

            for exit in self.exits(pos) {
                let cur_result = *distances.get(&exit).unwrap_or(&99999);
                if cur_distance < cur_result {
                    to_visit.push((exit, cur_distance + 1));
                }
            }
        }

        distances
    }

    pub fn calc_num_enclosed_tiles(&self) -> usize {
        let distances = self.calc_distances();
        let mut enclosed = 0;
        for y in 0..self.tiles.len() {
            let mut is_inside = false;
            let line = &self.tiles[y];
            for x in 0..line.len() {
                let is_loop_tile = distances.contains_key(&(x as i32, y as i32));
                if is_loop_tile && ['|', 'L', 'J'].contains(&line[x].ty) {
                    is_inside = !is_inside;
                }
                if is_inside && !is_loop_tile {
                    enclosed += 1;
                }
            }
        }
        enclosed
    }

    pub fn calc_max_distance(&self) -> usize {
        *self.calc_distances().values().max().unwrap()
    }

    pub fn exits(&self, pos: Pos) -> Vec<Pos> {
        if pos.0 < 0 || pos.1 < 0 {
            return vec![];
        }
        match self.tiles[pos.1 as usize][pos.0 as usize].ty {
            '|' => vec![(pos.0, pos.1 - 1), (pos.0, pos.1 + 1)],
            '-' => vec![(pos.0 - 1, pos.1), (pos.0 + 1, pos.1)],
            'L' => vec![(pos.0, pos.1 - 1), (pos.0 + 1, pos.1)],
            'J' => vec![(pos.0, pos.1 - 1), (pos.0 - 1, pos.1)],
            '7' => vec![(pos.0, pos.1 + 1), (pos.0 - 1, pos.1)],
            'F' => vec![(pos.0, pos.1 + 1), (pos.0 + 1, pos.1)],
            '.' => vec![],
            'S' => panic!("S"),
            _ => panic!("Unknown tile"),
        }
    }
}

fn parse_input(input: &str) -> Map {
    let lines = input.lines();
    let tiles = lines.into_iter()
        .map(|line| {
            line.chars().into_iter()
                .map(Tile::from)
                .collect()
        })
        .collect();

    let mut map = Map::from(tiles);
    map.fix_start_tile();
    map
}

fn main() {
    let input = include_str!("day10.txt");
    let map = parse_input(input);
    println!("max distance: {}", map.calc_max_distance());
    println!("num enclosed tiles: {}", map.calc_num_enclosed_tiles());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = r#".....
.S-7.
.|.|.
.L-J.
....."#;
        let map = parse_input(input);
        assert_eq!(map.calc_max_distance(), 4);
    }

    #[test]
    fn example2() {
        let input = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#;
        let map = parse_input(input);
        assert_eq!(map.calc_max_distance(), 8);
    }

    #[test]
    fn example3() {
        let input = r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."#;
        let map = parse_input(input);
        assert_eq!(map.calc_num_enclosed_tiles(), 4);
    }

    #[test]
    fn example4() {
        let input = r#".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."#;
        let map = parse_input(input);
        assert_eq!(map.calc_num_enclosed_tiles(), 8);
    }

    #[test]
    fn example5() {
        let input = r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"#;
        let map = parse_input(input);
        assert_eq!(map.calc_num_enclosed_tiles(), 10);
    }
}