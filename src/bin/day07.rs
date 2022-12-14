use std::fs;
use ndarray::Array;
use ndarray_stats::QuantileExt;

fn main() {

    let positions= Array::from_vec(get_positions());
    let max_val = positions.max().unwrap();
    let mut min_idx = 0;
    let mut current_min = i64::MAX;
    for pos_idx in 0..*max_val {
        let fuel = (&positions - pos_idx).mapv(i64::abs).map(|x| (x*(x+1))/2).sum();
        if fuel < current_min{
            min_idx = pos_idx;
            current_min = fuel;
        }
    }
    println!("Total fuel: {}", current_min);
    println!("Position: {}", positions[min_idx as usize]);

}

fn task01(){
    let positions= Array::from_vec(get_positions());
    let mut min_idx = 0;
    let mut current_min = i64::MAX;
    for pos_idx in 0..positions.len() {
        let fuel = (&positions - positions[pos_idx]).mapv(i64::abs).sum();
        if fuel < current_min{
            min_idx = pos_idx;
            current_min = fuel;
        }
    }
    println!("Total fuel: {}", current_min);
    println!("Position: {}", positions[min_idx]);
}

fn get_positions() -> Vec<i64>{
    let mut output: Vec<i64> = Vec::new();

    let filename = "./data/day07.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file").replace("\r", "");

    let entries: Vec<String> = contents.split(",").map(str::to_string).collect();

    for entry in &entries {
        output.push(entry.parse::<i64>().unwrap());
    }
    return output;
}