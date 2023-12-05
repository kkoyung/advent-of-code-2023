use std::io::{BufRead, BufReader};
use std::{fs::File, io};

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let mut sum: u32 = 0;

    for line in reader.lines() {
        let mut is_first = true;
        let mut first = 0;
        let mut last = 0;

        let line = line?;
        let in_str = line.as_str();
        for index in 0..line.len() {
            let num: Option<u32> = if let Ok(n) = in_str[index..index + 1].parse::<u32>() {
                Some(n)
            } else if in_str[index..].starts_with("one") {
                Some(1)
            } else if in_str[index..].starts_with("two") {
                Some(2)
            } else if in_str[index..].starts_with("three") {
                Some(3)
            } else if in_str[index..].starts_with("four") {
                Some(4)
            } else if in_str[index..].starts_with("five") {
                Some(5)
            } else if in_str[index..].starts_with("six") {
                Some(6)
            } else if in_str[index..].starts_with("seven") {
                Some(7)
            } else if in_str[index..].starts_with("eight") {
                Some(8)
            } else if in_str[index..].starts_with("nine") {
                Some(9)
            } else {
                None
            };

            if let Some(n) = num {
                if is_first {
                    first = n;
                    is_first = false;
                }
                last = n;
            }
        }

        sum += (first * 10) + last;
    }
    println!("{}", sum);

    Ok(())
}
