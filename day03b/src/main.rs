use std::fs;

#[derive(Debug)]
struct Num {
    num: u32,
    row: usize,
    start: usize,
    end: usize,
}

fn line_to_nums(line: &str, row: usize) -> Vec<Num> {
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
                    row,
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
            row,
            start,
            end: end + 1,
        });
    }

    nums
}

fn main() {
    let data = fs::read_to_string("input").expect("Unable to read file");
    let data = data.lines().collect::<Vec<&str>>();

    let nums: Vec<Num> = data
        .iter()
        .enumerate()
        .map(|(row, line)| line_to_nums(line, row))
        .flatten()
        .collect();

    let mut sum = 0;
    for (star_row, line) in data.iter().enumerate() {
        for (star_col, _) in line.chars().enumerate().filter(|(_, c)| *c == '*') {
            let matched: Vec<&Num> = nums
                .iter()
                .filter(|num| {
                    (num.row >= star_row - 1 && num.row <= star_row + 1)
                        && (num.start <= star_col + 1 && num.end + 1 > star_col)
                })
                .collect();
            if matched.len() == 2 {
                sum += matched[0].num * matched[1].num;
            }
        }
    }

    println!("{}", sum);
}
