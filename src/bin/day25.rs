use std::{fs};


fn main(){
    let mut cucumber = load_cucumbers("./data/day25.txt");
    let mut moves = 1;
    let mut total = 0;
    while moves > 0{
        let (ncucumber, nmoves) = do_step(&cucumber);
        cucumber = ncucumber.clone();
        moves = nmoves;
        total+=1;
    }
    println!("Task 1: {}", total);
}

fn do_step(cucumbers: &Vec<Vec<Option<Cucumber>>>) -> (Vec<Vec<Option<Cucumber>>>, u64){
    let nrows = cucumbers.len();
    let ncols = cucumbers[0].len();
    let mut next = cucumbers.clone();
    let mut total_moves = 0;
    for pass in 0..2 {
        let current = next.clone();
        for row in 0..nrows {
            for col in 0..ncols{
                match current[row][col] {
                    None => {}
                    Some(sc) => {
                        match sc.heard {
                            Heard::East => {
                                if pass == 0{
                                    let next_coords = get_next(&current[row][col], (row, col), (nrows, ncols));
                                    match current[next_coords.0][next_coords.1] {
                                        None => {
                                            next[next_coords.0][next_coords.1] = next[row][col];
                                            next[row][col] = None;
                                            total_moves += 1;
                                        }
                                        Some(_) => {}
                                    }
                                }
                            }
                            Heard::South => {
                                if pass == 1{
                                    let next_coords = get_next(&current[row][col], (row, col), (nrows, ncols));
                                    match current[next_coords.0][next_coords.1] {
                                        None => {
                                            next[next_coords.0][next_coords.1] = next[row][col];
                                            next[row][col] = None;
                                            total_moves += 1;
                                        }
                                        Some(_) => {}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    (next, total_moves)
}


fn get_next(cucumber: &Option<Cucumber>, location: (usize, usize), limits: (usize, usize)) -> (usize, usize){
    match cucumber {
        None => {location}
        Some(cucumber) => {
            match cucumber.heard{
                Heard::East => {
                    let row = location.0;
                    let mut col = location.1 + 1;
                    if col >= limits.1{
                        col = 0;
                    }
                    (row, col)
                }
                Heard::South => {
                    let mut row = location.0 + 1;
                    let col = location.1;
                    if row >= limits.0{
                        row = 0;
                    }
                    (row, col)
                }
            }
        }
    }
}

fn print_cucumbers(cucumber: &[Vec<Option<Cucumber>>]){
    for row in 0 .. cucumber.len(){
        for col in 0 .. cucumber[row].len(){
            match cucumber[row][col]{
                None => {print!(".")}
                Some(sc) => {
                    match sc.heard{
                        Heard::East => {print!(">")}
                        Heard::South => {print!("v")}
                    }
                }
            }
        }
        println!(" ");
    }
    println!(" ");
    println!(" ");
}

#[derive(Copy, Clone)]
enum Heard{
    East,
    South,
}

#[derive(Copy, Clone)]
struct Cucumber{
    heard: Heard,
}


fn load_cucumbers(path: &str) -> Vec<Vec<Option<Cucumber>>> {
    let contents = fs::read_to_string(path)
        .expect("Something went wrong reading the file").replace('\r', "");
    let mut output = Vec::new();
    for (_, line) in contents.trim().lines().enumerate() {
        let mut buffer = Vec::new();
        for (_, entry) in line.chars().enumerate() {
            match entry {
                '>' => buffer.push(Some(Cucumber{heard : Heard::East})),
                'v' => buffer.push(Some(Cucumber{heard : Heard::South})),
                _ => {buffer.push(None)},
            }
        }
        output.push(buffer.clone())
    }
    output
}