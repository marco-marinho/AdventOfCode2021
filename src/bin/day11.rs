use std::fs;
use ndarray::{Array, Ix2, Axis};

fn main() {
    task01();
    task02();
}

fn task01(){
    let mut matrix = get_matrix();
    let mut total_flashed = 0;
    for _step in 0..100 {
        total_flashed += time_step(&mut matrix);
    }
    println!("Number of flashes: {}", total_flashed);
}

fn task02(){
    let mut matrix = get_matrix();
    let rows = matrix.len_of(Axis(0));
    let cols = matrix.len_of(Axis(1));
    let total_octopuses = rows * cols;
    let mut target_time = 0;
    for step in 1..1000 {
        let flashed = time_step(&mut matrix);
        if flashed == total_octopuses as u32{
            target_time = step;
            break;
        }
    }
    println!("Timestep: {}", target_time);
}

fn time_step(matrix: &mut Array<u32, Ix2>) -> u32{
    *matrix += 1;
    let mut total_flashed = 0;
    loop {
        let flashed = flash(matrix);
        total_flashed += flashed;
        if flashed == 0 {
            break;
        }
    }
    return total_flashed;
}

fn flash(matrix: &mut Array<u32, Ix2>) -> u32{
    let mut flashed = 0;
    let rows = matrix.len_of(Axis(0));
    let cols = matrix.len_of(Axis(1));
    for row in 0..rows{
        for col in 0..cols{
            if matrix[[row, col]] >= 10 {
                flashed += 1;
                matrix[[row, col]] = 0;
                iluminate_neighbors(matrix, (row, col));
            }
        }
    }
    return flashed;
}

fn iluminate_neighbors(matrix: &mut Array<u32, Ix2>, coodinates: (usize, usize)) {
    let rows = matrix.len_of(Axis(0)) as i32;
    let cols = matrix.len_of(Axis(1)) as i32;
    for row_offset in -1..2 {
        for col_offset in -1..2 {
            let row = coodinates.0 as i32 + row_offset;
            let col = coodinates.1 as i32 + col_offset;
            if row >= 0 && row < rows && col >= 0 && col < cols {
                if matrix[[row as usize, col as usize]] != 0 {
                    matrix[[row as usize, col as usize]] += 1;
                }
            }
        }
    }
}

fn get_matrix() -> Array<u32, Ix2> {
    let filename = "./data/day11.txt";

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