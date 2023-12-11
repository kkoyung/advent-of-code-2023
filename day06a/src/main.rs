use std::fs;

fn main() {
    let data = fs::read_to_string("input").expect("Unable to read file");

    // Parse input
    let mut numbers = data.lines().map(|line| {
        line.split_once(":")
            .unwrap()
            .1
            .trim()
            .split_whitespace()
            .map(|num| num.parse::<u64>().unwrap())
            .collect::<Vec<u64>>()
    });
    let times: Vec<u64> = numbers.next().unwrap();
    let distances: Vec<u64> = numbers.next().unwrap();

    // Start calculation
    let result: u64 = times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance)| {
            // Let t = time, d = distance, p = pressing duration.
            // Since p*(t-p)>d, we know
            //  upper_bound_of_press_time = (t+sqrt(t^2-4d))/2  (exclusive)
            //  lower_bound_of_press_time = (t-sqrt(t^2-4d))/2  (exclusive)

            let inside_root = (time * time) - (4 * distance);
            if inside_root > 0 {
                let root = (inside_root as f64).sqrt();
                let upper_bound = ((*time as f64) + root) / 2.0; // exclusive
                let lower_bound = ((*time as f64) - root) / 2.0; // exclusive

                // number of valid choices
                //  = (upper_bound.ceil() as u64 - 1) - (lower_bound.floor() as u64 + 1) + 1
                (upper_bound.ceil() as u64) - (lower_bound.floor() as u64) - 1
            } else {
                0
            }
        })
        .product();

    println!("{}", result);
}
