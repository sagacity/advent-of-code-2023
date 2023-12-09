use itertools::Itertools;

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input.lines()
        .map(|line| {
            line.split(' ').map(|val| val.parse::<i64>().unwrap()).collect()
        })
        .collect()
}

fn predict_seq(seq: Vec<i64>) -> i64 {
    let mut seqs = vec![seq];
    loop {
        let seq = seqs.last().as_ref().unwrap().windows(2)
            .map(|a| a[1] - a[0])
            .collect::<Vec<_>>();
        let done = seq.iter().all_equal();
        seqs.push(seq);
        if done {
            break;
        }
    }

    seqs.iter().rev()
        .map(|seq| seq.last().unwrap())
        .sum()
}

fn predict_rev_seq(seq: Vec<i64>) -> i64 {
    let mut seqs = vec![seq];
    loop {
        let seq = seqs.last().as_ref().unwrap().windows(2)
            .map(|a| a[1] - a[0])
            .collect::<Vec<_>>();
        let done = seq.iter().all_equal();
        seqs.push(seq);
        if done {
            let mut last = vec![];
            for _ in 0..seqs.last().unwrap().len() - 1 {
                last.push(0);
            }
            seqs.push(last);
            break;
        }
    }

    let firsts = seqs.iter().rev()
        .map(|seq| *seq.first().unwrap())
        .collect::<Vec<_>>();
    let mut total = 0;
    for t in firsts {
        total = t - total;
    }
    total
}

fn main() {
    let input = include_str!("day9.txt");
    let seqs = parse_input(input);
    let predictions = seqs.clone().into_iter().map(predict_seq).sum::<i64>();
    println!("prediction: {}", predictions);
    let predictions = seqs.into_iter().map(predict_rev_seq).sum::<i64>();
    println!("prediction: {}", predictions);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;
        let seqs = parse_input(input);
        let predictions = seqs.clone().into_iter().map(predict_seq).sum::<i64>();
        assert_eq!(predictions, 114);
        let predictions = seqs.into_iter().map(predict_rev_seq).sum::<i64>();
        assert_eq!(predictions, 2);
    }
}