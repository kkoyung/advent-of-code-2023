use std::{collections::HashSet, fs};

fn main() {
    let data = fs::read_to_string("input").expect("Unable to read file");
    let map: Vec<Vec<char>> = data
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

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
    let expanded_map: Vec<Vec<char>> = map
        .iter()
        .enumerate()
        .map(|(row_ind, row)| {
            let new_row: Vec<char> = row
                .iter()
                .enumerate()
                .map(|(col_ind, c)| {
                    if non_empty_col.contains(&col_ind) {
                        vec![c]
                    } else {
                        vec![c, c]
                    }
                })
                .flatten()
                .map(|&c| c)
                .collect();
            if empty_row.contains(&row_ind) {
                vec![new_row.clone(), new_row]
            } else {
                vec![new_row]
            }
        })
        .flatten()
        .collect();

    // Locate galaxies
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    for (row_ind, row) in expanded_map.iter().enumerate() {
        for (col_ind, c) in row.iter().enumerate() {
            if *c == '#' {
                galaxies.push((row_ind, col_ind));
            }
        }
    }

    // Calculate length
    let mut sum: usize = 0;
    for (i, first) in galaxies.iter().enumerate() {
        for second in galaxies.iter().skip(i + 1) {
            sum += first.0.abs_diff(second.0);
            sum += first.1.abs_diff(second.1);
        }
    }

    println!("{}", sum);
}
