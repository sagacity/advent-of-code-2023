use itertools::Itertools;

fn hash_csv(input: &str) -> usize {
    input.split(',').map(hash).sum::<usize>()
}

fn hash_map(input: &str) -> usize {
    let mut maps: Vec<Vec<(String, usize)>> = vec![];
    for _ in 0..256 {
        maps.push(vec![]);
    }

    for item in input.split(',') {
        if item.contains('=') {
            let split = item.split('=').collect_vec();
            let (id, value) = (split[0], split[1]);
            let value = value.parse::<usize>().unwrap();
            let box_id = hash(id);
            let bx = &mut maps[box_id];

            let mut found = false;
            for i in 0..bx.len() {
                if bx[i].0 == id {
                    bx[i].1 = value;
                    found = true;
                    break;
                }
            }
            if !found {
                bx.push((id.to_string(), value));
            }
        } else {
            let split = item.split('-').collect_vec();
            let (id, _) = (split[0], split[1]);
            let box_id = hash(id);
            let bx = &mut maps[box_id];
            for i in 0..bx.len() {
                if bx[i].0 == id {
                    bx.remove(i);
                    break;
                }
            }
        }
    }

    let mut total = 0;
    for i in 0..256 {
        let items = &maps[i];
        if !items.is_empty() {
            println!("Box {i}");
            for (slot, (id, value)) in items.iter().enumerate() {
                println!("{} = {}", id, *value);
                total += (i + 1) * (slot + 1) * *value;
            }
        }
    }

    total
}

fn hash(input: &str) -> usize {
    let mut result = 0;
    for ch in input.chars() {
        result = (result + (ch as u8 as usize)) * 17;
        result &= 255;
    }
    result
}

fn main() {
    let input = include_str!("day15.txt");
    println!("hash: {}", hash_csv(input));
    println!("hash_map: {}", hash_map(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn example() {
        assert_eq!(hash_csv("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"), 1320);
    }

    #[test]
    fn example2() {
        assert_eq!(hash_map("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"), 145);
    }
}
