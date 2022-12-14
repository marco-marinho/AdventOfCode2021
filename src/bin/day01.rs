use std::fs;

fn main() {
    println!("Task 01 result: {}", task01());
    println!("Task 02 result: {}", task02());
}

fn task01() -> i32{
    let filename = "./data/day01.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file").replace("\r", "");

    let split = contents.split("\n");

    let mut last = -1;
    let mut count = 0;

    for line in split {
        if last == -1 {
            last = line.parse::<i32>().unwrap();
        }
        else{
            let current = line.parse::<i32>().unwrap();
            if current > last{
                count+=1;
            }
            last = current;
        }
    }
    return count;
}

fn task02() -> i32{
    let filename = "./data/day01.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let split = contents.split("\n");
    let vec = split.collect::<Vec<&str>>();
    let mut count = 0;

    for index in 0..vec.len()-3 {
        let mut group1 = 0;
        let mut group2 = 0;
        for offset in 0..3{
            group1 += vec[index+offset].replace("\r", "").parse::<i32>().unwrap();
        }
        for offset in 1..4{
            group2 += vec[index+offset].replace("\r", "").parse::<i32>().unwrap();
        }
        if group2 > group1{
            count += 1;
        }
    }
    return count;
}
