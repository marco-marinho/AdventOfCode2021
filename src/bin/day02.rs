use std::fs;

fn main() {

    task01();
    task02();

}

fn task01(){
    let filename = "./data/day02.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file").replace("\r", "");

    let commands = contents.split("\n");

    let mut horizontal = 0;
    let mut depth = 0;

    for command in commands {
        let order = command.split(" ").collect::<Vec<&str>>();
        let direction = order[0];
        let amount = order[1].parse::<i32>().unwrap();

        if direction == "forward"{
            horizontal += amount;
        }
        else if direction == "down" {
            depth += amount;
        }
        else{
            depth -= amount;
        }
    }

    println!("Task 01 result {}", horizontal*depth);
}

fn task02(){

    let filename = "./data/day02.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file").replace("\r", "");

    let commands = contents.split("\n");

    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for command in commands {
        let order = command.split(" ").collect::<Vec<&str>>();
        let direction = order[0];
        let amount = order[1].parse::<i32>().unwrap();

        if direction == "forward"{
            horizontal += amount;
            depth += aim * amount;
        }
        else if direction == "down" {
            aim += amount;
        }
        else{
            aim -= amount;
        }
    }

    println!("Task 02 result {}", horizontal*depth);

}