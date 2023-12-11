use std::fs;

fn main() {
    let data = fs::read_to_string("input").expect("Unable to read file");

    // Parse input
    let mut numbers = data.lines().map(|line| {
        line.split_once(":")
            .unwrap()
            .1
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
            .parse::<u64>()
            .unwrap()
    });
    let time: u64 = numbers.next().unwrap();
    let distance: u64 = numbers.next().unwrap();

    // Start calculation
    let inside_root = (time * time) - (4 * distance);
    let result = if inside_root > 0 {
        let root = (inside_root as f64).sqrt();
        let upper_bound = ((time as f64) + root) / 2.0; // exclusive
        let lower_bound = ((time as f64) - root) / 2.0; // exclusive
        (upper_bound.ceil() as u64) - (lower_bound.floor() as u64) - 1
        // == (upper_bound.ceil() as u64 - 1) - (lower_bound.floor() as u64 + 1) + 1
    } else {
        0
    };

    println!("{}", result);
}
