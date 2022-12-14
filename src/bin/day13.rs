use std::fs;
use ndarray::{Array, Axis, Ix2, s};

fn main() {
    task01();
    task02();
}

fn task02(){
    let mut paper = get_paper();
    let folds = get_folds();
    for fold_idx in 0..folds.len(){
        paper = fold(paper, folds[fold_idx]);
    }
    print_code(paper);
}

fn task01(){
    let paper = get_paper();
    let folds = get_folds();
    let new_paper = fold(paper, folds[0]);
    println!("Number of dots: {}", count_dots(new_paper));
}

fn print_code(paper: Array<u8, Ix2>){
    for row in 0..paper.len_of(Axis(0)){
        let mut line = String::new();
        for col in 0..paper.len_of(Axis(1)){
            if paper[[row, col]] > 0 {line += "##"}
            else{ line += "  "}
        }
        println!("{}", line);
    }
}

fn count_dots(paper: Array<u8, Ix2>) -> i32{
    let dots = paper.iter().fold(0, |acc, x|  if *x > 0 as u8 { acc + 1 } else { acc });
    return dots;
}

fn fold(paper:Array<u8, Ix2>, fold:(char, usize)) -> Array<u8, Ix2>{
    if fold.0 == 'y'{
        let mut slice_1 = paper.slice(s![..fold.1, ..]).to_owned();
        let rows_1 = slice_1.len_of(Axis(0));
        let mut slice_2 = paper.slice(s![fold.1 + 1.., ..]).to_owned();
        slice_2.invert_axis(Axis(0));
        let rows_2 = slice_2.len_of(Axis(0));
        if rows_1 < rows_2 { slice_1 = pad_rows(slice_1, rows_2); }
        if rows_2 < rows_1 { slice_2 = pad_rows(slice_2, rows_1); }
        slice_1 + slice_2
    }
    else{
        let slice_1 = paper.slice(s![.., ..fold.1]).to_owned();
        let mut slice_2 = paper.slice(s![.., fold.1 + 1..]).to_owned();
        slice_2.invert_axis(Axis(1));
        slice_1 + slice_2
    }
}

fn pad_rows(paper: Array<u8, Ix2>, target_rows: usize)-> Array<u8, Ix2>{
    let cols = paper.len_of(Axis(1));
    let rows = paper.len_of(Axis(0));
    let mut output = Array::zeros((target_rows, cols));
    let diff = target_rows - rows;
    output.slice_mut(s![diff.., ..]).assign(&paper);
    return output;
}

fn get_paper() -> Array<u8, Ix2> {
    let filename = "./data/day13.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file").replace("\r", "");
    let entries: Vec<String> = contents.split("\n").map(str::to_string).collect();
    let mut coordinates:Vec<(usize, usize)> = Vec::new();
    for entry in &entries {
        if !entry.contains(","){
            continue;
        }
        let line: Vec<usize> = entry.split(",").map(|x|str::parse::<usize>(x).unwrap()).collect();
        coordinates.push((line[0], line[1]))
    }
    let num_rows = coordinates.clone().into_iter().map(|(v, _)| v).fold(0, std::cmp::max);
    let num_cols = coordinates.clone().into_iter().map(|(_,v)|v).fold(0, std::cmp::max);
    let mut output = Array::zeros((num_cols + 1, num_rows + 1));
    for coordinate in &coordinates {
        output[[coordinate.1, coordinate.0]] = 1;
    }
    return output;
}

fn get_folds() -> Vec<(char, usize)>{
    let filename = "./data/day13.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file").replace("\r", "");
    let entries: Vec<String> = contents.split("\n").map(str::to_string).collect();
    let mut folds:Vec<(char, usize)> = Vec::new();
    for entry in &entries {
        if !entry.contains("="){
            continue;
        }
        let line: Vec<String> = entry.replace("fold along ", "").split("=").map(str::to_string).collect();
        folds.push((line[0].chars().next().unwrap(), line[1].parse::<usize>().unwrap()))
    }
    return folds;
}