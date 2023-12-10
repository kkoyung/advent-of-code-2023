use std::fs;

#[derive(Debug)]
struct Range {
    destination: u64,
    source: u64,
    length: u64,
}

impl Range {
    fn to_destination(&self, source: u64) -> Option<u64> {
        if self.source <= source && source < self.source + self.length {
            Some(self.destination + (source - self.source))
        } else {
            None
        }
    }
}

struct Mapping {
    ranges: Vec<Range>,
}

impl Mapping {
    fn to_destination(&self, source: u64) -> u64 {
        let try_match: Vec<u64> = self
            .ranges
            .iter()
            .map(|range| range.to_destination(source))
            .flatten()
            .collect();
        if try_match.len() >= 2 {
            panic!(
                "Overlapped ranges\nsource: {:?}\nmapping: {:?}",
                source, self.ranges
            )
        } else if try_match.len() == 1 {
            try_match[0]
        } else {
            source
        }
    }
}

fn main() {
    let data = fs::read_to_string("input").expect("Unable to read file");
    let mut lines = data.lines();

    // Read seeds
    let seeds: Vec<u64> = lines
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|seed| seed.parse::<u64>().unwrap())
        .collect();

    // Read mappings
    let mut mappings: Vec<Mapping> = Vec::new();
    while let Some(line) = lines.next() {
        if !line.ends_with("map:") {
            continue;
        }

        let mut mapping = Mapping { ranges: Vec::new() };
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
            mapping.ranges.push(Range {
                destination: nums[0],
                source: nums[1],
                length: nums[2],
            })
        }
        mappings.push(mapping);
    }

    // Start mapping
    let result = seeds
        .iter()
        .map(|seed| {
            mappings
                .iter()
                .fold(*seed, |source, mapping| mapping.to_destination(source))
        })
        .min()
        .unwrap();

    println!("{}", result);
}
