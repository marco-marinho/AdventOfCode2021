use std::fs;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

fn main() {
    task01();
    task02();
}

fn task02(){
    let connections = get_connections();
    let paths = get_paths(connections, true);
    println!("Total paths: {}", paths.len());
}

fn task01(){
    let connections = get_connections();
    let paths = get_paths(connections, false);
    println!("Total paths: {}", paths.len());
}

fn check_repeatable(input: &String) -> bool {
    if input.to_lowercase().eq(input) || input.eq(&"start") || input.eq(&"end"){
        return false;
    }
    return true;
}

fn check_small_repeatable(input: &Vec<String>, next: &String) -> bool {
    if  next.eq(&"start") || next.eq(&"end") {
        return false;
    }
    let mut small: Vec<String> = Vec::new();
    for entry in input{
        if entry.to_lowercase().eq(entry) && !entry.eq(&"start") && !entry.eq(&"end"){
            small.push(entry.clone());
        }
    }
    if has_unique_elements(small){
        return true;
    }
    return false;
}

fn has_unique_elements<T>(iter: T) -> bool
    where
        T: IntoIterator,
        T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

fn get_paths(connections: HashMap<String, Vec<String>>, allow_repeat: bool) -> Vec<Vec<String>>{
    let mut output = Vec::new();
    let mut first: Vec<String> = Vec::new();
    first.push("start".to_string());
    output.push(first);
    loop {
        let next = get_next_connections(&output, &connections, allow_repeat);
        if next != output{
            output = next;
        }
        else{
            break;
        }
    }
    return output;
}

fn get_next_connections(paths: &Vec<Vec<String>>, connections: &HashMap<String, Vec<String>>, allow_repeat: bool) -> Vec<Vec<String>>{
    let mut output = Vec::new();
    for path in paths{
        let current = path.clone();
        let key = current.last().unwrap();
        if key.eq("end"){
            output.push(current);
            continue;
        }
        let possible_next = connections.get(key).unwrap();
        for next in possible_next{
            let mut buff = current.clone();
            if check_repeatable(next) {
                buff.push(next.clone());
                output.push(buff);
            }
            else {
                let mut small_repeat = false;
                if allow_repeat {
                    small_repeat = check_small_repeatable(&buff, next);
                }
                if !buff.contains(next) || small_repeat {
                    buff.push(next.clone());
                    output.push(buff);
                }
            }
        }
    }
    return output;
}

fn get_connections() -> HashMap<String, Vec<String>> {
    let filename = "./data/day12.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file").replace("\r", "");
    let entries: Vec<String> = contents.split("\n").map(str::to_string).collect();
    let mut output = HashMap::new();
    for entry in entries {
        let line: Vec<String> = entry.split("-").map(str::to_string).collect();
        output.entry(line[0].clone()).or_insert_with(Vec::new).push(line[1].clone());
        output.entry(line[1].clone()).or_insert_with(Vec::new).push(line[0].clone());
    }
    return output;
}