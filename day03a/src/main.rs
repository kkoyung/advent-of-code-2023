use std::fs;

#[derive(Debug)]
struct Num {
    num: u32,
    start: usize,
    end: usize,
}

fn line_to_nums(line: &str) -> Vec<Num> {
    let mut nums = Vec::new();

    let mut num = 0;
    let mut start = 0;
    let mut end = 0;
    let mut inside = false;
    for (i, c) in line.chars().enumerate() {
        if let Some(digit) = c.to_digit(10) {
            if inside {
                num = num * 10 + digit;
                end = i;
            } else {
                inside = true;
                num = digit;
                start = i;
                end = i;
            }
        } else {
            if inside {
                nums.push(Num {
                    num,
                    start,
                    end: end + 1,
                });
                inside = false;
            } else {
                continue;
            }
        }
    }
    if inside {
        nums.push(Num {
            num,
            start,
            end: end + 1,
        });
    }

    nums
}

fn main() {
    let data = fs::read_to_string("input").expect("Unable to read file");

    let mut data = data
        .lines()
        .map(|data| Some(data))
        .collect::<Vec<Option<&str>>>();
    let width = data[0].unwrap().len();
    data.push(None);
    data.insert(0, None);

    let sum: u32 = data
        .windows(3)
        .map(|lines| {
            let nums = line_to_nums(lines[1].unwrap());
            nums.iter()
                .map(|num| {
                    let safe_start = if num.start > 0 { num.start - 1 } else { 0 };
                    let safe_end = if num.end < width - 1 {
                        num.end + 1
                    } else {
                        width
                    };
                    let num_of_symbols = lines
                        .iter()
                        .flatten()
                        .map(|line| line.get(safe_start..safe_end).unwrap())
                        .fold(String::new(), |mut boundary, edge| {
                            boundary.push_str(edge);
                            boundary
                        })
                        .chars()
                        .filter(|&c| !(c.is_digit(10) || c == '.'))
                        .count();
                    if num_of_symbols > 0 {
                        num.num
                    } else {
                        0
                    }
                })
                .sum::<u32>()
        })
        .sum();

    println!("{}", sum);
}
