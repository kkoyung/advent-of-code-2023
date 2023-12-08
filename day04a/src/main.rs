use std::{fs, collections::BTreeSet};

fn main() {
    let data = fs::read_to_string("input").expect("Unable to read file");

    let sum = data.lines().map(|line| {
        let (_prefix, content) = line.split_once(":").unwrap();
        let (winning_str, having_str) = content.split_once("|").unwrap();

        let winning = winning_str
            .trim()
            .split_whitespace()
            .map(|num_str| num_str.parse::<u32>().unwrap())
            .collect::<BTreeSet<u32>>();

        having_str
            .trim()
            .split_whitespace()
            .map(|num_str| num_str.parse::<u32>().unwrap())
            .filter(|num| winning.contains(num))
            .fold(1 as u32, |acc, _| acc * 2)/2
    }).sum::<u32>();

    println!("{}", sum);
}
