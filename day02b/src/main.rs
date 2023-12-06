use std::cmp;
use std::io::{BufRead, BufReader};
use std::{fs::File, io};

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let mut sum: u32 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let (_prefix, content) = line.trim().split_once(":").unwrap();

        let min_rgb = content
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
            .fold((0, 0, 0), |min_rgb: (u32, u32, u32), rgb| {
                (
                    cmp::max(min_rgb.0, rgb.0),
                    cmp::max(min_rgb.1, rgb.1),
                    cmp::max(min_rgb.2, rgb.2),
                )
            });

        sum += min_rgb.0 * min_rgb.1 * min_rgb.2;
    }
    println!("{}", sum);

    Ok(())
}
