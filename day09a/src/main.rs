use std::fs;

fn main() {
    let data = fs::read_to_string("input").expect("Unable to read file");

    let histories: Vec<Vec<i32>> = data
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let result: i32 = histories.iter().map(|history| extrapolate(history)).sum();

    println!("{}", result);
}

fn extrapolate(history: &Vec<i32>) -> i32 {
    let mut sequence = history.clone();
    let mut result = 0;

    while !sequence.iter().all(|&num| num == 0) {
        result += sequence.last().unwrap();
        sequence = sequence.windows(2).map(|nums| nums[1] - nums[0]).collect();
    }

    result
}
