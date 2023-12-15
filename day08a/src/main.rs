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
    for line in lines {
        let splitted: Vec<&str> = line.split(&[' ', '=', '(', ',', ')']).collect();
        map.insert(splitted[0], (splitted[4], splitted[6]));
    }

    // Start running
    let mut location = "AAA";
    let mut count: usize = 0;
    while location.ne("ZZZ") {
        location = intructions.iter().fold(location, |current, step| {
            let next_locations = map.get(current).unwrap();
            match step {
                Intruction::Left => next_locations.0,
                Intruction::Right => next_locations.1,
            }
        });
        count += intructions.len();
    }

    println!("{}", count);
}
