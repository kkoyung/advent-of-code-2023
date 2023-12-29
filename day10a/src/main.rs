use std::fs;

#[derive(PartialEq, Eq, Clone, Debug)]
struct Location {
    i: usize,
    j: usize,
}

#[derive(Clone, Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug)]
struct Probe {
    location: Location,
    direction: Direction,
}

impl Probe {
    fn move_one_step(&mut self, map: &Vec<Vec<char>>) {
        let depth = map.len();
        let width = map[0].len();

        match self.direction {
            Direction::North => {
                if self.location.i > 0 {
                    self.location.i -= 1;
                    self.direction = match map[self.location.i][self.location.j] {
                        '|' => Direction::North,
                        '7' => Direction::West,
                        'F' => Direction::East,
                        _ => panic!("Broken pipe"),
                    }
                } else {
                    panic!("Reach the edge")
                }
            }
            Direction::South => {
                if self.location.i < depth - 1 {
                    self.location.i += 1;
                    self.direction = match map[self.location.i][self.location.j] {
                        '|' => Direction::South,
                        'J' => Direction::West,
                        'L' => Direction::East,
                        _ => panic!("Broken pipe"),
                    }
                } else {
                    panic!("Reach the edge")
                }
            }
            Direction::West => {
                if self.location.j > 0 {
                    self.location.j -= 1;
                    self.direction = match map[self.location.i][self.location.j] {
                        '-' => Direction::West,
                        'L' => Direction::North,
                        'F' => Direction::South,
                        _ => panic!("Broken pipe"),
                    }
                } else {
                    panic!("Reach the edge")
                }
            }
            Direction::East => {
                if self.location.j < width - 1 {
                    self.location.j += 1;
                    self.direction = match map[self.location.i][self.location.j] {
                        '-' => Direction::East,
                        'J' => Direction::North,
                        '7' => Direction::South,
                        _ => panic!("Broken pipe"),
                    }
                } else {
                    panic!("Reach the edge")
                }
            }
        }
    }

}

fn main() {
    let data = fs::read_to_string("input").expect("Unable to read file");

    let map: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    let depth = map.len();
    let width = map[0].len();

    // Find starting point S
    let starting: Location = map
        .iter()
        .map(|row| row.iter().position(|&c| c == 'S'))
        .enumerate()
        .find_map(|(i, found)| match found {
            Some(j) => Some(Location { i, j }),
            None => None,
        })
        .unwrap();

    println!("starting: ({}, {})", starting.i, starting.j);

    // Scan the surrounding of the starting point S
    let mut surrounding = Vec::new();
    if starting.i > 0 {
        match map[starting.i - 1][starting.j] {
            '|' | '7' | 'F' => surrounding.push(Direction::North),
            _ => {}
        };
    }
    if starting.i < depth - 1 {
        match map[starting.i + 1][starting.j] {
            '|' | 'J' | 'L' => surrounding.push(Direction::South),
            _ => {}
        };
    }
    if starting.j > 0 {
        match map[starting.i][starting.j - 1] {
            '-' | 'L' | 'F' => surrounding.push(Direction::West),
            _ => {}
        };
    }
    if starting.j < width - 1 {
        match map[starting.i][starting.j + 1] {
            '-' | 'J' | '7' => surrounding.push(Direction::East),
            _ => {}
        };
    }
    if surrounding.len() != 2 {
        panic!("The animal is not in a closed loop")
    }

    // Create two probes
    let mut probes = vec![
        Probe {
            location: starting.clone(),
            direction: surrounding[0].clone(),
        },
        Probe {
            location: starting.clone(),
            direction: surrounding[1].clone(),
        },
    ];

    // Launch the probes
    let mut step: usize = 0;
    loop {
        probes[0].move_one_step(&map);
        probes[1].move_one_step(&map);
        step += 1;
        if probes[0].location == probes[1].location {
            break;
        }
    }

    println!("{}", step);
}
