use std::fs;

#[derive(Debug)]
struct SeedRange {
    start: u64,
    length: u64,
}

#[derive(Debug)]
struct Rule {
    destination: u64,
    start: u64, // source
    length: u64,
}

#[derive(Debug)]
struct Mapping {
    rules: Vec<Rule>,
}

impl Mapping {
    fn map_seed(&self, seed: &SeedRange) -> Vec<SeedRange> {
        let mut splitting_points: Vec<u64> = Vec::new();
        splitting_points.push(seed.start);
        for rule in &self.rules {
            if seed.start < rule.start && rule.start < seed.start + seed.length {
                splitting_points.push(rule.start);
            }
            if seed.start < rule.start + rule.length
                && rule.start + rule.length < seed.start + seed.length
            {
                splitting_points.push(rule.start + rule.length);
            }
        }
        splitting_points.push(seed.start + seed.length);
        splitting_points.sort();
        splitting_points.dedup();

        let result = splitting_points
            .windows(2)
            .map(|pair| SeedRange {
                start: pair[0],
                length: pair[1] - pair[0],
            })
            .map(|mut seed| {
                for rule in &self.rules {
                    if rule.start <= seed.start
                        && seed.start + seed.length <= rule.start + rule.length
                    {
                        seed.start = seed.start + rule.destination - rule.start;
                        break;
                    }
                }
                seed
            })
            .collect();

        result
    }
}

fn main() {
    let data = fs::read_to_string("input").expect("Unable to read file");
    let mut lines = data.lines();

    // Read seeds
    let seeds: Vec<SeedRange> = lines
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|num| num.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
        .as_slice()
        .chunks_exact(2)
        .map(|num_pair| SeedRange {
            start: num_pair[0],
            length: num_pair[1],
        })
        .collect();

    // Read mappings
    let mut mappings: Vec<Mapping> = Vec::new();
    while let Some(line) = lines.next() {
        if !line.ends_with("map:") {
            continue;
        }

        let mut mapping = Mapping { rules: Vec::new() };
        while let Some(line) = lines.next() {
            if line.eq("") {
                break;
            }

            let nums: Vec<u64> = line
                .split_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
                .collect();
            if nums.len() < 2 {
                panic!("Corrupted map");
            }
            mapping.rules.push(Rule {
                destination: nums[0],
                start: nums[1],
                length: nums[2],
            })
        }
        mappings.push(mapping);
    }

    // Start mapping
    let result = mappings
        .iter()
        .fold(seeds, |seeds, mapping| {
            seeds
                .iter()
                .map(|seed| mapping.map_seed(seed))
                .flatten()
                .collect()
        })
        .iter()
        .map(|seed| seed.start)
        .min()
        .unwrap();

    println!("{}", result);
}
