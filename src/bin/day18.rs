use std::{fs};

fn main() {
    task01();
    task02();
}

fn task02(){
    let numbers = get_numbers();
    let mut max_mag = 0;
    for idx_start in 0..numbers.len(){
        for idx in idx_start..numbers.len(){
            let mut added = add(&numbers[idx_start], &numbers[idx]);
            reduce(&mut added);
            let mag = magnitude(added);
            if mag > max_mag { max_mag = mag; }
            let mut added = add(&numbers[idx], &numbers[idx_start]);
            reduce(&mut added);
            let mag = magnitude(added);
            if mag > max_mag { max_mag = mag; }
        }
    }
    println!("{}", max_mag);
}

fn task01(){
    let numbers = get_numbers();
    let mut current = numbers[0].clone();
    for idx in 1..numbers.len(){
        let mut added = add(&current, &numbers[idx]);
        reduce(&mut added);
        current = added.clone();
    }
    let mag = magnitude(current);
    println!("{}", mag);
}

fn magnitude(input: Vec<Value>) -> u32{
    let mut input = input.clone();
    loop {
        for idx in 0..input.len() - 1 {
            if input[idx].1 == input[idx + 1].1 {
                let mag = 3 * input[idx].0 + 2 * input[idx + 1].0;
                let depth = input[idx].1;
                input.remove(idx);
                input.remove(idx);
                input.insert(idx, Value(mag, depth - 1));
                break
            }
        }
        if input.len() == 1{
            break;
        }
    }
    return input[0].0;
}

fn reduce(input: &mut Vec<Value>){
    loop{
        let exploded = explode_once(input);
        if exploded{continue;}
        let splited = split_once(input);
        if splited{continue};
        break;
    }
}

fn add(first: &Vec<Value>, second: &Vec<Value>) -> Vec<Value>{
    let mut first = first.clone();
    first = first.iter().map(|x| Value(x.0, x.1 + 1)).collect();
    let mut second = second.clone();
    second = second.iter().map(|x| Value(x.0, x.1 + 1)).collect();
    first.extend(second.iter());
    return first;
}

fn explode_once(input: &mut Vec<Value>) -> bool{
    let mut idx_to_remove: usize = 0;
    let mut exploded = false;
    let mut exploded_depth = 0;
    for idx in 0..input.len(){
        if input[idx].1 >= 5 && input[idx + 1].1 >= 5{
            if idx > 0 {
                input[idx - 1].0 += input[idx].0;
            }
            if idx < input.len() - 2{
                input[idx + 2].0 += input[idx + 1].0
            }
            exploded = true;
            idx_to_remove = idx;
            exploded_depth = input[idx].1;
            break;
        }
    }
    if exploded{
        input.remove(idx_to_remove);
        input.remove(idx_to_remove);
        input.insert(idx_to_remove, Value(0, exploded_depth-1));
    }
    return exploded;
}

fn split_once(input: &mut Vec<Value>) -> bool{
    let mut idx_to_split: u8 = 0;
    let mut splited = false;
    let mut split_vals = (0,0);
    let mut split_depth = 0;
    for idx in 0..input.len(){
        if input[idx].0 > 9 {
            split_vals.0 = (input[idx].0 as f32 / 2f32).floor() as u32;
            split_vals.1 = (input[idx].0 as f32 / 2f32).ceil() as u32;
            split_depth = input[idx].1 + 1;
            idx_to_split = idx as u8;
            splited = true;
            break;
        }
    }
    if splited{
        input.remove(idx_to_split as usize);
        input.insert(idx_to_split as usize, Value(split_vals.1, split_depth));
        input.insert(idx_to_split as usize, Value(split_vals.0, split_depth));
    }
    return splited;
}

fn get_numbers() -> Vec<Vec<Value>>{
    let filename = "./data/day18.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file").replace("\r", "");
    let lines: Vec<String> = contents.split("\n").map(str::to_string).collect();
    let mut output: Vec<Vec<Value>> = Vec::new();
    for line in lines {
        output.push(parse_line(&line));
    }
    return output;
}

fn parse_line(input: &String) -> Vec<Value>{
    let mut current_depth = 0;
    let mut output: Vec<Value> = Vec::new();
    for entry in input.chars(){
        if entry == '[' {
            current_depth += 1;
        }
        if entry == ']' {
            current_depth -= 1;
        }
        if entry.is_numeric(){
            output.push(Value(entry.to_digit(10).unwrap(), current_depth));
        }
    }
    return output;
}

#[derive(Debug, Clone, Copy)]
struct Value(u32, u32);