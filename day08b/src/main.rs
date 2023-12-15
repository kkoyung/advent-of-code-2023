use num::integer::lcm;
use std::{collections::HashMap, fs};

enum Intruction {
    Left,
    Right,
}

fn main() {
    let data = fs::read_to_string("input").expect("Unable to read file");
    let mut lines = data.lines();

    // Parse instruction line
    let intructions: Vec<Intruction> = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Intruction::Left,
            'R' => Intruction::Right,
            _ => panic!("The intruction contains step neither 'L' nor 'R'"),
        })
        .collect();
    lines.next(); // skip the empty line

    // Parse map
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut locations: Vec<&str> = Vec::new();
    for line in lines {
        let splitted: Vec<&str> = line.split(&[' ', '=', '(', ',', ')']).collect();
        map.insert(splitted[0], (splitted[4], splitted[6]));

        if splitted[0].ends_with("A") {
            locations.push(splitted[0]);
        }
    }
    println!("Starting locations: {:?}", locations);

    // Start running for each starting location
    let counts: Vec<usize> = locations
        .iter()
        .map(|&starting_location| {
            let mut location = starting_location;
            let mut count: usize = 0;
            while !location.ends_with("Z") {
                location = intructions.iter().fold(location, |current, step| {
                    let next_locations = map.get(current).unwrap();
                    match step {
                        Intruction::Left => next_locations.0,
                        Intruction::Right => next_locations.1,
                    }
                });
                count += intructions.len();
            }
            count
        })
        .collect();
    println!("counts: {:?}", counts);

    // Least common multiple of steps
    let result = counts.iter().fold(1, |acc, &next| lcm(acc, next));
    println!("{}", result);
}
