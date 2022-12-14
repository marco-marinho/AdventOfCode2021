use std::collections::HashSet;
use std::fs;

use ndarray::{Array, Axis, Ix2};

fn main() {
    task01();
    task02();
}

fn task02() {
    let matrix = get_matrix();
    let rows = matrix.len_of(Axis(0));
    let cols = matrix.len_of(Axis(1));
    let mut basins: Vec<u32> = Vec::new();
    let mut valleys: Vec<(usize, usize)> = Vec::new();
    for row in 0..rows {
        for col in 0..cols {
            let element = matrix[[row, col]];
            if row > 0 {
                if matrix[[row - 1, col]] <= element {
                    continue;
                }
            }
            if row < rows - 1 {
                if matrix[[row + 1, col]] <= element {
                    continue;
                }
            }
            if col > 0 {
                if matrix[[row, col - 1]] <= element {
                    continue;
                }
            }
            if col < cols - 1 {
                if matrix[[row, col + 1]] <= element {
                    continue;
                }
            }
            valleys.push((row, col));
        }
    }
    for valley in valleys {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let size = check_basin(&matrix, valley, &mut visited);
        basins.push(size);
    }
    basins.sort_by(|a, b| b.cmp(a));
    println!("{}", basins[0] * basins[1] * basins[2]);
}

fn check_basin(matrix: &Array<u32, Ix2>, coordinates: (usize, usize), visited: &mut HashSet<(usize, usize)>) -> u32 {
    let rows = matrix.len_of(Axis(0));
    let cols = matrix.len_of(Axis(1));
    let row = coordinates.0;
    let col = coordinates.1;
    let mut sum_neightbors: u32 = 0;
    let element = matrix[[coordinates.0, coordinates.1]];
    if element == 9 || visited.contains(&coordinates) {
        return 0;
    }
    visited.insert(coordinates);
    if row > 0 {
        sum_neightbors += check_basin(matrix, (row - 1, col), visited);
    }
    if row < rows - 1 {
        sum_neightbors += check_basin(matrix, (row + 1, col), visited);
    }
    if col > 0 {
        sum_neightbors += check_basin(matrix, (row, col - 1), visited);
    }
    if col < cols - 1 {
        sum_neightbors += check_basin(matrix, (row, col + 1), visited);
    }
    return 1 + sum_neightbors;
}

fn task01() {
    let matrix = get_matrix();
    let rows = matrix.len_of(Axis(0));
    let cols = matrix.len_of(Axis(1));
    let mut risk: Vec<u32> = Vec::new();
    for row in 0..rows {
        for col in 0..cols {
            let element = matrix[[row, col]];
            if row > 0 {
                if matrix[[row - 1, col]] <= element {
                    continue;
                }
            }
            if row < rows - 1 {
                if matrix[[row + 1, col]] <= element {
                    continue;
                }
            }
            if col > 0 {
                if matrix[[row, col - 1]] <= element {
                    continue;
                }
            }
            if col < cols - 1 {
                if matrix[[row, col + 1]] <= element {
                    continue;
                }
            }
            risk.push(element);
        }
    }
    let height_sum = risk.iter().sum::<u32>() + risk.len() as u32;
    println!("{}", height_sum);
}

fn get_matrix() -> Array<u32, Ix2> {
    let filename = "./data/day09.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file").replace("\r", "");

    let entries: Vec<String> = contents.split("\n").map(str::to_string).collect();
    let rows = entries.len();
    let cols = entries[0].len();
    let mut output = Array::zeros((rows, cols));

    for row in 0..rows {
        let temp = &entries[row];
        for col in 0..cols {
            output[[row, col]] = temp.chars().nth(col).unwrap().to_digit(10).unwrap();
        }
    }
    return output;
}