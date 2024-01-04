use std::{collections::HashSet, fs};

fn main() {
    let data = fs::read_to_string("input").expect("Unable to read file");
    let mut map: Vec<Vec<char>> = data
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let width = map[0].len();
    const EXPAND_RATIO: usize = 1000000;

    // Find empty row and non-empty columns
    let empty_row: HashSet<usize> = map
        .iter()
        .enumerate()
        .filter(|(_ind, row)| row.iter().all(|&c| c == '.'))
        .map(|(ind, _row)| ind)
        .collect();
    let non_empty_col: HashSet<usize> = map
        .iter()
        .map(|row| {
            row.iter()
                .enumerate()
                .filter_map(|(ind, c)| if *c == '#' { Some(ind) } else { None })
                .collect::<HashSet<usize>>()
        })
        .flatten()
        .collect();

    // Expand the map
    for (row_ind, row) in map.iter_mut().enumerate() {
        if empty_row.contains(&row_ind) {
            *row = vec!['-'; width];
        }
        for (col_ind, c) in row.iter_mut().enumerate() {
            if !non_empty_col.contains(&col_ind) {
                if *c == '-' {
                    *c = '+';
                } else {
                    *c = '|';
                }
            }
        }
    }

    // Locate galaxies
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    for (row_ind, row) in map.iter().enumerate() {
        for (col_ind, c) in row.iter().enumerate() {
            if *c == '#' {
                galaxies.push((row_ind, col_ind));
            }
        }
    }

    // Calculate the sum
    let mut sum: usize = 0;
    for (i, first) in galaxies.iter().enumerate() {
        for second in galaxies.iter().skip(i + 1) {
            let mut length: usize = 0;
            let row_top = std::cmp::min(first.0, second.0);
            let row_bottom = std::cmp::max(first.0, second.0);
            let col_left = std::cmp::min(first.1, second.1) + 1;
            let col_right = std::cmp::max(first.1, second.1) + 1;

            for c in &map[row_top][col_left..col_right] {
                if *c == '|' {
                    length += EXPAND_RATIO;
                } else {
                    length += 1;
                }
            }
            for row in &map[row_top..row_bottom] {
                if row[col_right - 1] == '-' {
                    length += EXPAND_RATIO;
                } else {
                    length += 1;
                }
            }

            sum += length;
        }
    }

    println!("{}", sum);
}
