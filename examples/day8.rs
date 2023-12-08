use std::collections::BTreeMap;

fn num_moves_to_reach(input: &str) -> usize {
    let mut lines = input.lines();
    let steps = lines.next().unwrap().chars().collect::<Vec<_>>();
    let _ = lines.next();
    let mut text_nodes = vec![];
    for line in lines {
        let split = line.split(' ').collect::<Vec<_>>();
        let node_id = split[0];
        let exits = (split[2][1..4].to_string(), split[3][0..3].to_string());
        text_nodes.push((node_id, exits));
    }
    let mut nodes: BTreeMap<&str, (usize, usize)> = BTreeMap::new();
    for (text_node_name, (l, r)) in &text_nodes {
        let mut lidx = None;
        let mut ridx = None;
        for i in 0..text_nodes.len() {
            if text_nodes[i].0 == l {
                lidx = Some(i);
            }
            if text_nodes[i].0 == r {
                ridx = Some(i);
            }
        }

        nodes.insert(*text_node_name, (lidx.unwrap(), ridx.unwrap()));
    }

    let mut cur_poses = vec![];
    for node in nodes.keys() {
        if node.ends_with('A') {
            cur_poses.push(*node);
        }
    }
    println!("{:?}", cur_poses);

    let mut total_steps = vec![];
    for cur_pos in &mut cur_poses {
        let mut num_steps = 0;
        while !cur_pos.ends_with('Z') {
            let (l_index, r_index) = nodes[cur_pos];
            let index = match steps[num_steps % steps.len()] {
                'L' => l_index,
                'R' => r_index,
                _ => panic!()
            };
            *cur_pos = text_nodes[index].0;
            num_steps += 1;
        }
        total_steps.push(num_steps);
    }
    println!("{:?}", total_steps);
    total_steps.into_iter().fold(1, lcm)
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn main() {
    let input = include_str!("day8.txt");
    println!("num_moves: {}", num_moves_to_reach(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;
        assert_eq!(num_moves_to_reach(input), 6);
    }

    #[test]
    fn example2() {
        let input = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;
        assert_eq!(num_moves_to_reach(input), 6);
    }
}