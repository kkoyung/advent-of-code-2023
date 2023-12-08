use std::{cmp, collections::BTreeSet, fs};

fn main() {
    let data = fs::read_to_string("input").expect("Unable to read file");
    let num_of_games = data.lines().count();

    let sum = data
        .lines()
        .map(|line| {
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
                .count()
        })
        .enumerate()
        .fold(
            vec![1; num_of_games],
            |mut num_of_cards, (index, matching)| {
                for i in index + 1..cmp::min(index + 1 + matching, num_of_games) {
                    num_of_cards[i] += num_of_cards[index];
                }

                num_of_cards
            },
        )
        .iter()
        .sum::<usize>();

    println!("{}", sum);
}
