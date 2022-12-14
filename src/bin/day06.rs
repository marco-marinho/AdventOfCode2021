use std::fs;
use std::collections::HashMap;

fn main() {

    task01();
    task02();

}

fn task01(){
    let mut fishes = get_fish();
    let days = 80;
    for _day in 0..days {
        for fish in 0..fishes.len() {
            let result = fishes[fish].tick();
            if !result.is_none() {
                fishes.push(result.unwrap());
            }
        }
    }
    println!("{}", fishes.len());
}

fn task02(){
    let mut fishes = get_fish_map();
    let days = 256;

    for _day in 0..days {
        let mut buff: HashMap<u8, u64> = HashMap::new();
        for idx in fishes.clone().keys() {
            let x = *fishes.get(idx).unwrap();
            if *idx == 0 {
                *buff.entry(6).or_insert(0) += x;
                *buff.entry(8).or_insert(0) = x;
            } else {
                *buff.entry(idx - 1).or_insert(0) += x;
            }
        }
        fishes = buff.clone();
    }
    let num_fish = fishes.into_values().collect::<Vec<u64>>().iter().sum::<u64>();
    println!("{}", num_fish);
}

struct LatternFish{
    days: u8
}

impl LatternFish{
    pub fn tick(&mut self) -> Option<LatternFish> {
        if self.days == 0 {
            self.days = 6;
            return Some(LatternFish{days: 8});
        } else {
            self.days -= 1;
            return None;
        }
    }
}

fn get_fish() -> Vec<LatternFish>{

    let mut output: Vec<LatternFish> = Vec::with_capacity(10000000000);

    let filename = "./data/day06.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file").replace("\r", "");

    let fishes: Vec<String> = contents.split(",").map(str::to_string).collect();

    for fish in &fishes{
        let new_fish  = LatternFish{days: fish.parse::<u8>().unwrap()};
        output.push(new_fish);
    }
    return output;
}

fn get_fish_map() -> HashMap<u8, u64>{

    let mut output: HashMap<u8, u64> = HashMap::new();

    let filename = "./data/day06.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file").replace("\r", "");

    let fishes: Vec<String> = contents.split(",").map(str::to_string).collect();

    for fish in &fishes{
        *output.entry(fish.parse::<u8>().unwrap()).or_insert(0) += 1;
    }
    return output;

}