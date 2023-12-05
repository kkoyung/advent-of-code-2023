use std::{fs::File, io};
use std::io::{BufReader, BufRead};

fn main() -> io::Result<()>{
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let mut sum: u32 = 0;
    const RADIX: u32 = 10;

    for line in reader.lines() {
        let mut is_first = true;
        let mut first = 0;
        let mut last = 0;

        for c in line?.chars() {
            if let Some(num) = c.to_digit(RADIX) {
                if is_first {
                    first = num;
                    is_first = false;
                }
                last = num;
            }
        }
        sum += (first * 10) + last;
    }
    println!("{}", sum);

    Ok(())
}
