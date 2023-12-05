use std::cmp::{max, min};

#[derive(Debug)]
struct Almanac {
    seeds: Vec<(usize, usize)>,
    mappings: Vec<Mapping>,
}

#[derive(Debug, Default)]
struct Mapping {
    ranges: Vec<MappingRange>
}

#[derive(Debug, Default)]
struct MappingRange {
    destination_start: usize,
    source_start: usize,
    length: usize,
}

fn parse(input: &str, use_seed_ranges: bool) -> Almanac {
    let mut lines = input.lines();

    let seeds = lines.next().unwrap();
    let mut seeds = seeds.split(" ").skip(1).map(|seed| (usize::from_str_radix(seed, 10).unwrap(), 1)).collect::<Vec<_>>();
    if use_seed_ranges {
        let mut new_seeds  = vec![];

        let mut seed = seeds.into_iter();
        while let Some((start, _)) = seed.next() {
            let (range, _) = seed.next().unwrap();
            new_seeds.push((start, range));
        }
        seeds = new_seeds;
    }
    let mut mappings = vec![];

    let _ = lines.next();
    let mut mapping = Mapping::default();
    while let Some(line) = lines.next() {
        let split = line.split(" ").collect::<Vec<_>>();
        match split.len() {
            1 => {
                mappings.push(mapping);
                mapping = Mapping::default();
            }
            2 => {},
            3 => {
                mapping.ranges.push(MappingRange {
                    destination_start: usize::from_str_radix(split[0], 10).unwrap(),
                    source_start: usize::from_str_radix(split[1], 10).unwrap(),
                    length: usize::from_str_radix(split[2], 10).unwrap(),
                });
            },
            _ => panic!()
        }
    }
    mappings.push(mapping);

    Almanac {
        seeds,
        mappings
    }
}

fn map_seeds(almanac: Almanac) -> Vec<usize> {
    let mut seeds = almanac.seeds.clone();
    for (seed_start, seed_range) in &mut seeds {
        for mapping in &almanac.mappings {
            for range in &mapping.ranges {
                println!("Seed: {:?} in range: {:?}", (&seed_start, &seed_range), (range.source_start, range.length));
                if *seed_start <= (range.source_start + range.length - 1) && (*seed_start + *seed_range - 1) >= range.source_start {
                    println!("IN RANGE");
                    let min_start = max(*seed_start, range.source_start);
                    println!("Map to: {}", range.destination_start);
                    println!("--> {}", (min_start - *seed_start) + range.destination_start);
                    *seed_start = range.destination_start + (min_start - *seed_start);
                    break;
                }
            }
        }
    }
    seeds.into_iter().map(|(seed, _)| seed).collect()
}

fn main() {
    let input = include_str!("day5.txt");
    let almanac = parse(input, false);
    let mut seeds = map_seeds(almanac);
    seeds.sort();
    println!("min seed: {}", seeds.first().unwrap());

    let almanac = parse(input, true);
    let mut seeds = map_seeds(almanac);
    seeds.sort();
    println!("min seed: {}", seeds.first().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        //let input = r#"seeds: 79
        let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;
        /*let almanac = parse(input, false);
        let mut seeds = map_seeds(almanac);
        seeds.sort();
        assert_eq!(seeds.first().unwrap(), &35);*/

        let almanac = parse(input, true);
        let mut seeds = map_seeds(almanac);
        seeds.sort();
        assert_eq!(seeds.first().unwrap(), &46);
    }
}