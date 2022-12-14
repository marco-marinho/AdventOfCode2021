use std::fs;
use itertools::min;
use ndarray::{Array2, Array1, Axis, s};

fn main() {
    task01();
    task02();
}

fn task02(){
    let map = get_map();
    let map_extended = expand_map(&map);
    let dist = djkistras(&map_extended, (0, 0));
    println!("{}", dist);
}

fn task01(){
    let map = get_map();
    let dist = djkistras(&map, (0, 0));
    println!("{}", dist);
}

fn djkistras(weights: &Array2<u32>, source: (usize, usize)) -> Array2<u32>{
    let rows = weights.len_of(Axis(0));
    let cols = weights.len_of(Axis(1));
    let mut dist: Array2<u32> = Array2::from_elem((rows, cols), u32::MAX);
    let mut visited: Array2<bool> = Array2::from_elem((rows, cols), false);
    let mut origin_map: Array2<(usize, usize)> = Array2::from_elem((rows, cols), (usize::MAX, usize::MAX));
    dist[[source.0, source.1]] = 0;
    let mut row: usize = source.0;
    let mut col: usize = source.1;
    loop{
        if row < rows-1 && !visited[[row+1,col]]{
            if dist[[row+1,col]] > weights[[row+1,col]] + dist[[row,col]]{
                dist[[row+1,col]] = weights[[row+1,col]] + dist[[row,col]];
                origin_map[[row+1,col]] = (row, col);
            }
        }
        if row > 0 && !visited[[row-1,col]]{
            if dist[[row - 1,col]] > weights[[row-1,col]] + dist[[row,col]]{
                dist[[row - 1,col]] = weights[[row-1,col]] + dist[[row,col]];
                origin_map[[row - 1 ,col]] = (row, col);
            }
        }
        if col < cols-1 && !visited[[row,col + 1]]{
            if dist[[row,col + 1]] > weights[[row,col + 1]] + dist[[row,col]]{
                dist[[row,col + 1]] = weights[[row,col + 1]] + dist[[row,col]];
                origin_map[[row,col + 1]] = (row, col);
            }
        }
        if col > 0 && !visited[[row,col - 1]]{
            if dist[[row,col - 1]] > weights[[row,col - 1]] + dist[[row,col]]{
                dist[[row,col - 1]] = weights[[row,col - 1]] + dist[[row,col]];
                origin_map[[row, col-1]] = (row, col);
            }
        }
        visited[[row, col]] = true;
        let minpost = arg_min(&dist, &visited);
        row = minpost.0;
        col = minpost.1;
        if row == rows - 1 && col == cols-1{
            break;
        }
    }
    return dist;
}

fn arg_min(input: &Array2<u32>, visited: &Array2<bool>) -> (usize, usize) {
    let rows = input.len_of(Axis(0));
    let cols = input.len_of(Axis(1));
    let mut min = u32::MAX;
    let mut min_row: usize = 0;
    let mut min_col: usize = 0;
    for row in 0..rows{
        for col in 0..cols{
            if input[[row, col]] < min && !visited[[row, col]]{
                min = input[[row, col]];
                min_row = row;
                min_col = col;
            }
        }
    }
    return (min_row, min_col);
}

fn expand_map(map: &Array2<u32>) -> Array2<u32>{
    let num_rows = map.len_of(Axis(0));
    let num_cols = map.len_of(Axis(1));
    let original = map.clone();
    let mut output = Array2::zeros((num_rows * 5, num_cols * 5));
    for riter in 0..5 as usize {
        if riter == 0 {
            output.slice_mut(s![0..num_rows, 0..num_cols]).assign(&original);
        }
        else {
            let mut temp_slice = output.slice_mut(s![num_rows * (riter - 1)  .. num_rows * riter, 0..num_cols]).to_owned();
            temp_slice += 1;
            temp_slice = temp_slice.mapv(|x| if x > 9 { 1 } else { x });
            output.slice_mut(s![num_rows * riter .. num_rows * (riter + 1), 0..num_cols]).assign(&temp_slice);
        }
        for iter in 1..5 as usize{
            let mut temp_slice = output.slice_mut(s![num_rows * riter .. num_rows * (riter + 1), num_cols * (iter - 1) .. num_cols * iter]).to_owned();
            temp_slice += 1;
            temp_slice = temp_slice.mapv(|x| if x > 9 { 1 } else { x });
            output.slice_mut(s![num_rows * riter .. num_rows * (riter + 1), num_cols * iter .. num_cols * (iter + 1)]).assign(&temp_slice);
        }
    }
    return output;
}

fn get_map() -> Array2<u32> {
    let filename = "./data/day15.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file").replace("\r", "");
    let entries: Vec<String> = contents.split("\n").map(str::to_string).collect();
    let rows = entries.len();
    let cols = entries[0].len();
    let mut output: Array2<u32> = Array2::zeros((rows, cols));
    for row in 0..rows {
        let line_digits: Vec<u32> = entries[row].chars().map(|x| x.to_digit(10).unwrap()).collect();
        output.slice_mut(s![row, ..]).assign(&Array1::from_vec(line_digits));
    }
    return output;
}
