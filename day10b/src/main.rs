use std::{collections::VecDeque, fs};

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
    rotation: i32, // +1: rotate 90 degree clockwise, -1:rotate 90 degree anti-clockwise
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
                        '7' => {
                            self.rotation -= 1;
                            Direction::West
                        }
                        'F' => {
                            self.rotation += 1;
                            Direction::East
                        }
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
                        'J' => {
                            self.rotation += 1;
                            Direction::West
                        }
                        'L' => {
                            self.rotation -= 1;
                            Direction::East
                        }
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
                        'L' => {
                            self.rotation += 1;
                            Direction::North
                        }
                        'F' => {
                            self.rotation -= 1;
                            Direction::South
                        }
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
                        'J' => {
                            self.rotation -= 1;
                            Direction::North
                        }
                        '7' => {
                            self.rotation += 1;
                            Direction::South
                        }
                        _ => panic!("Broken pipe"),
                    }
                } else {
                    panic!("Reach the edge")
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Mark {
    Left,
    Right,
    Loop,
    Unknown,
}

struct MarkMap {
    width: usize,
    depth: usize,
    map: Vec<Vec<Mark>>,
    queue: VecDeque<Location>,
}

impl MarkMap {
    fn spread(&mut self, location: &Location, direction: Direction, mark: Mark) {
        match direction {
            Direction::North => {
                if location.i > 0 && self.map[location.i - 1][location.j] == Mark::Unknown {
                    self.map[location.i - 1][location.j] = mark;
                    self.queue.push_back(Location {
                        i: location.i - 1,
                        j: location.j,
                    });
                }
            }
            Direction::South => {
                if location.i < self.depth - 1
                    && self.map[location.i + 1][location.j] == Mark::Unknown
                {
                    self.map[location.i + 1][location.j] = mark;
                    self.queue.push_back(Location {
                        i: location.i + 1,
                        j: location.j,
                    });
                }
            }
            Direction::West => {
                if location.j > 0 && self.map[location.i][location.j - 1] == Mark::Unknown {
                    self.map[location.i][location.j - 1] = mark;
                    self.queue.push_back(Location {
                        i: location.i,
                        j: location.j - 1,
                    });
                }
            }
            Direction::East => {
                if location.j < self.width - 1
                    && self.map[location.i][location.j + 1] == Mark::Unknown
                {
                    self.map[location.i][location.j + 1] = mark;
                    self.queue.push_back(Location {
                        i: location.i,
                        j: location.j + 1,
                    });
                }
            }
        }
    }

    fn clear_queue(&mut self) {
        while let Some(location) = self.queue.pop_front() {
            let mark = self.map[location.i][location.j];
            self.spread(&location, Direction::North, mark.clone());
            self.spread(&location, Direction::South, mark.clone());
            self.spread(&location, Direction::West, mark.clone());
            self.spread(&location, Direction::East, mark.clone());
        }
    }
}

fn main() {
    let data = fs::read_to_string("input").expect("Unable to read file");

    let mut map: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
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

    // Close the loop
    match (&surrounding[0], &surrounding[1]) {
        (Direction::North, Direction::North) => panic!("surrounding[0] == surrounding[1]"),
        (Direction::North, Direction::South) => map[starting.i][starting.j] = '|',
        (Direction::North, Direction::West) => map[starting.i][starting.j] = 'J',
        (Direction::North, Direction::East) => map[starting.i][starting.j] = 'L',

        (Direction::South, Direction::North) => map[starting.i][starting.j] = '|',
        (Direction::South, Direction::South) => panic!("surrounding[0] == surrounding[1]"),
        (Direction::South, Direction::West) => map[starting.i][starting.j] = '7',
        (Direction::South, Direction::East) => map[starting.i][starting.j] = 'F',

        (Direction::West, Direction::North) => map[starting.i][starting.j] = 'J',
        (Direction::West, Direction::South) => map[starting.i][starting.j] = '7',
        (Direction::West, Direction::West) => panic!("surrounding[0] == surrounding[1]"),
        (Direction::West, Direction::East) => map[starting.i][starting.j] = '-',

        (Direction::East, Direction::North) => map[starting.i][starting.j] = 'L',
        (Direction::East, Direction::South) => map[starting.i][starting.j] = 'F',
        (Direction::East, Direction::West) => map[starting.i][starting.j] = '-',
        (Direction::East, Direction::East) => panic!("surrounding[0] == surrounding[1]"),
    }

    // Create a probes
    let mut probe = Probe {
        location: starting.clone(),
        direction: surrounding[0].clone(),
        rotation: 0,
    };

    let mut mark_map = MarkMap {
        width,
        depth,
        map: vec![vec![Mark::Unknown; width]; depth],
        queue: VecDeque::new(),
    };

    // Launch the probe to mark the loop
    loop {
        mark_map.map[probe.location.i][probe.location.j] = Mark::Loop;
        probe.move_one_step(&map);
        if probe.location == starting {
            break;
        }
    }

    // Launch the probe to mark left/right
    loop {
        match probe.direction {
            Direction::North => {
                mark_map.spread(&probe.location, Direction::West, Mark::Left);
                mark_map.spread(&probe.location, Direction::East, Mark::Right);
            }
            Direction::South => {
                mark_map.spread(&probe.location, Direction::East, Mark::Left);
                mark_map.spread(&probe.location, Direction::West, Mark::Right);
            }
            Direction::West => {
                mark_map.spread(&probe.location, Direction::South, Mark::Left);
                mark_map.spread(&probe.location, Direction::North, Mark::Right);
            }
            Direction::East => {
                mark_map.spread(&probe.location, Direction::North, Mark::Left);
                mark_map.spread(&probe.location, Direction::South, Mark::Right);
            }
        }

        let direction = probe.direction.clone();
        let pipe = map[probe.location.i][probe.location.j];
        match (direction, pipe) {
            (Direction::North, '|') => {
                mark_map.spread(&probe.location, Direction::West, Mark::Left);
                mark_map.spread(&probe.location, Direction::East, Mark::Right);
            }
            (Direction::North, 'J') => {
                mark_map.spread(&probe.location, Direction::East, Mark::Right);
                mark_map.spread(&probe.location, Direction::South, Mark::Right);
            }
            (Direction::North, 'L') => {
                mark_map.spread(&probe.location, Direction::West, Mark::Left);
                mark_map.spread(&probe.location, Direction::South, Mark::Left);
            }
            (Direction::South, '|') => {
                mark_map.spread(&probe.location, Direction::East, Mark::Left);
                mark_map.spread(&probe.location, Direction::West, Mark::Right);
            }
            (Direction::South, '7') => {
                mark_map.spread(&probe.location, Direction::East, Mark::Left);
                mark_map.spread(&probe.location, Direction::North, Mark::Left);
            }
            (Direction::South, 'F') => {
                mark_map.spread(&probe.location, Direction::West, Mark::Right);
                mark_map.spread(&probe.location, Direction::North, Mark::Right);
            }
            (Direction::West, '-') => {
                mark_map.spread(&probe.location, Direction::South, Mark::Left);
                mark_map.spread(&probe.location, Direction::North, Mark::Right);
            }
            (Direction::West, 'J') => {
                mark_map.spread(&probe.location, Direction::East, Mark::Left);
                mark_map.spread(&probe.location, Direction::South, Mark::Left);
            }
            (Direction::West, '7') => {
                mark_map.spread(&probe.location, Direction::East, Mark::Right);
                mark_map.spread(&probe.location, Direction::North, Mark::Right);
            }
            (Direction::East, '-') => {
                mark_map.spread(&probe.location, Direction::North, Mark::Left);
                mark_map.spread(&probe.location, Direction::South, Mark::Right);
            }
            (Direction::East, 'L') => {
                mark_map.spread(&probe.location, Direction::West, Mark::Right);
                mark_map.spread(&probe.location, Direction::South, Mark::Right);
            }
            (Direction::East, 'F') => {
                mark_map.spread(&probe.location, Direction::West, Mark::Left);
                mark_map.spread(&probe.location, Direction::North, Mark::Left);
            }
            _ => {}
        }

        probe.move_one_step(&map);
        if probe.location == starting {
            break;
        }
    }

    // Clear queue
    mark_map.clear_queue();

    // Get result
    let result;
    if probe.rotation == 8 {
        // two complete clockwise rotations => right side is inner side
        result = mark_map.map.iter().flatten().filter(|&&mark| mark == Mark::Right).count();
    } else if probe.rotation == -8 {
        // two complete anti-clockwise rotations => left side is inner side
        result = mark_map.map.iter().flatten().filter(|&&mark| mark == Mark::Left).count();
    } else {
        panic!("Not two complete rotation")
    }

    println!("{}", result);
}
