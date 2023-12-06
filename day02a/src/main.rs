use std::io::{BufRead, BufReader};
use std::{fs::File, io};

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let mut sum: u32 = 0;
    const RED: u32 = 12;
    const GREEN: u32 = 13;
    const BLUE: u32 = 14;

    for line in reader.lines() {
        let line = line.unwrap();
        let (prefix, content) = line.trim().split_once(":").unwrap();
        let id = prefix
            .split_once(" ")
            .unwrap()
            .1
            .to_string()
            .parse::<u32>()
            .unwrap();

        let is_pass = content
            .split(";")
            .into_iter()
            .map(|subset| {
                let mut rgb: (u32, u32, u32) = (0, 0, 0);
                for num_color in subset.trim().split(",") {
                    let (num, color) = num_color.trim().split_once(" ").unwrap();
                    if color.eq("red") {
                        rgb.0 = num.to_string().parse::<u32>().unwrap();
                    } else if color.eq("green") {
                        rgb.1 = num.to_string().parse::<u32>().unwrap();
                    } else if color.eq("blue") {
                        rgb.2 = num.to_string().parse::<u32>().unwrap();
                    }
                }
                rgb
            })
            .fold(true, |is_pass, rgb| {
                if is_pass && rgb.0 <= RED && rgb.1 <= GREEN && rgb.2 <= BLUE {
                    true
                } else {
                    false
                }
            });

        if is_pass {
            sum += id;
        }
    }
    println!("{}", sum);

    Ok(())
}
