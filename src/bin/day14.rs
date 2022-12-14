use std::collections::HashMap;
use std::fs;
use itertools::{Itertools, MinMaxResult};

fn main() {
    task01();
    task02();
}

fn task02() {
    let (table, template) = get_insertion_table();
    let (mut current_string, mut current_result) = template_to_hashmap(template);
    for _idx in 0..40 {
        let (new_string, new_result) = insert_and_count(&current_string, &current_result, &table);
        current_string = new_string;
        current_result = new_result;
    }
    let result = current_result.values().minmax();
    match result {
        MinMaxResult::NoElements => { println!("Empty count.") }
        MinMaxResult::OneElement(single) => { println!("Single element with count: {}", single) }
        MinMaxResult::MinMax(min, max) => {
            println!("Min and max elements: {min}, {max}.\nDifference: {difference}", min = min, max = max, difference = max - min)
        }
    }
}

fn insert_and_count(istring: &HashMap<String, u64>, count: &HashMap<char, u64>, table: &HashMap<String, char>) -> (HashMap<String, u64>, HashMap<char, u64>) {
    let mut ostring = HashMap::new();
    let mut ocount = count.clone();
    for key in istring.keys() {
        let char_to_insert = table.get(key).unwrap();
        *ocount.entry(*char_to_insert).or_insert(0) += istring.get(key).unwrap();
        let mut slice = key.clone();
        slice.insert(1, *char_to_insert);
        *ostring.entry(slice[0..2].to_string()).or_insert(0) += istring.get(key).unwrap();
        *ostring.entry(slice[1..].to_string()).or_insert(0) += istring.get(key).unwrap();
    }
    return (ostring, ocount);
}

fn template_to_hashmap(input: String) -> (HashMap<String, u64>, HashMap<char, u64>) {
    let mut string: HashMap<String, u64> = HashMap::new();
    let mut result: HashMap<char, u64> = HashMap::new();
    for idx in 0..input.len() - 1 {
        let slice = input[idx..idx + 2].to_string();
        *string.entry(slice).or_insert(0) += 1;
    }
    for char in input.chars() {
        *result.entry(char).or_insert(0) += 1;
    }
    return (string, result);
}

fn task01() {
    let (table, template) = get_insertion_table();
    let mut output = template.clone();
    for _idx in 0..10 {
        output = insert_chars(output, &table);
    }
    let count = count(output);
    let result = count.values().minmax();
    match result {
        MinMaxResult::NoElements => { println!("Empty count.") }
        MinMaxResult::OneElement(single) => { println!("Single element with count: {}", single) }
        MinMaxResult::MinMax(min, max) => {
            println!("Min and max elements: {min}, {max}.\nDifference: {difference}", min = min, max = max, difference = max - min)
        }
    }
}

fn insert_chars(input: String, table: &HashMap<String, char>) -> String {
    let mut output: String = String::with_capacity(usize::MAX / 2000000000);
    output += &input[0..1];
    for idx in 0..input.len() - 1 {
        let mut slice = input[idx..idx + 2].to_string();
        let char_to_insert = table.get(&slice).unwrap();
        slice.insert(1, *char_to_insert);
        output += &slice[1..];
    }
    return output;
}

fn count(input: String) -> HashMap<char, u64> {
    let mut count: HashMap<char, u64> = HashMap::new();
    for char in input.chars().into_iter() {
        *count.entry(char).or_insert(0) += 1;
    }
    return count;
}

fn get_insertion_table() -> (HashMap<String, char>, String) {
    let filename = "./data/day14.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file").replace("\r", "");
    let entries: Vec<String> = contents.split("\n").map(str::to_string).collect();
    let template = entries[0].clone();
    let mut table: HashMap<String, char> = HashMap::new();
    for entry in &entries {
        if !entry.contains("->") {
            continue;
        }
        let line: Vec<String> = entry.split(" -> ").map(str::to_string).collect();
        table.insert(line[0].clone(), line[1].chars().next().unwrap());
    }
    return (table, template);
}