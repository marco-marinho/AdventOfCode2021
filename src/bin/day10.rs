use std::fs;
use std::collections::{HashSet, HashMap};

fn main() {
    task01();
    task02();
}

fn task02() {
    let entries = get_entries();
    let openers = get_openers();
    let closers = get_closers();
    let substitution = get_substitution_table();
    let mut incomplete: Vec<String> = Vec::new();
    let mut scores: Vec<u64> = Vec::new();

    for entry in entries{
        let corrupt = check_if_corrupt(&entry, &openers, &closers);
        if corrupt == 0 {
            incomplete.push(entry.clone());
        }
    }
    for entry in incomplete {
        let mut still_open = get_remaining_open(&entry, &openers, &closers);
        let needed = get_needed(&mut still_open, &substitution);
        let score = get_score(needed);
        scores.push(score);
    }
    scores.sort();
    println!("Score: {}", scores[scores.len()/2]);
}

fn get_needed(input: &mut Vec<char>, table: &HashMap<char,char>) -> Vec<char>{
    let mut output:Vec<char> = Vec::new();
    while input.len() > 0 {
        let key = input.pop().unwrap();
        output.push(*table.get(&key).unwrap());
    }
    return output;
}

fn get_remaining_open(input: &String, openers: &HashSet<char>, _closers: &HashSet<char>) -> Vec<char> {
    let mut open_queue:Vec<char> = Vec::new();
    let elements = input.chars().collect::<Vec<char>>();
    for element in elements {
        if openers.contains(&element) {
            open_queue.push(element);
        }
        else{
            open_queue.pop();
        }
    }
    return open_queue;
}

fn get_score(input: Vec<char>) -> u64{
    let mut score = 0;
    for element in input{
        score = score * 5;
        if element == ')'
        {
            score += 1;
        }
        if element == ']'
        {
            score += 2;
        }
        if element == '}'
        {
            score += 3;
        }
        if element == '>'
        {
            score += 4;
        }
    }
    return score;
}

fn task01(){
    let entries = get_entries();
    let openers = get_openers();
    let closers = get_closers();
    let mut result = 0;

    for entry in entries{
        let corrupt = check_if_corrupt(&entry, &openers, &closers);
        result += corrupt;
    }
    println!("Score: {}", result);
}

fn check_if_corrupt(input: &String, openers: &HashSet<char>, _closers: &HashSet<char>) -> u32 {
    let mut open_queue:Vec<char> = Vec::new();
    let elements = input.chars().collect::<Vec<char>>();
    for element in elements {
        if openers.contains(&element){
            open_queue.push(element);
        }
        else{
            let queue_last = open_queue.last().unwrap();
            if element == ')' && *queue_last != '('
            {
                return 3;
            }
            if element == ']' && *queue_last != '['
            {
                return 57;
            }
            if element == '}' && *queue_last != '{'
            {
                return 1197;
            }
            if element == '>' && *queue_last != '<'
            {
                return 25137;
            }
            open_queue.pop();
        }
    }
    return 0;
}

fn get_substitution_table() -> HashMap<char, char>{
    let mut output: HashMap<char, char> = HashMap::new();
    output.insert('(', ')');
    output.insert('[', ']');
    output.insert('{', '}');
    output.insert('<', '>');
    return output;
}

fn get_openers() -> HashSet<char>{
    let mut openers: HashSet<char> = HashSet::new();
    openers.insert('(');
    openers.insert('{');
    openers.insert('[');
    openers.insert('<');
    return openers;
}

fn get_closers() -> HashSet<char>{
    let mut closers: HashSet<char> = HashSet::new();
    closers.insert(')');
    closers.insert('}');
    closers.insert(']');
    closers.insert('>');
    return closers;
}

fn get_entries() -> Vec<String> {

    let filename = "./data/day10.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file").replace("\r", "");

    let entries: Vec<String> = contents.split("\n").map(str::to_string).collect();

    return entries;
}