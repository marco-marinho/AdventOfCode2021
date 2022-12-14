use std::fs;
use regex::Regex;
use itertools::Itertools;

fn main() {

    task01();
    task02();

}

fn task02(){
    let numbers = get_numbers();
    let mut boards = boards();
    let mut time_of_victory = vec![0; boards.len()];
    let mut time = 0;

    for number in &numbers{
        let mut victory: bool;
        for board_idx in 0..boards.len() {
            if time_of_victory[board_idx] == 0 {
                boards[board_idx].update_board(*number);
                victory = boards[board_idx].check_victory();
                if victory {
                    time_of_victory[board_idx] = time;
                    boards[board_idx].victory_number = *number;
                }
            }
        }
        time += 1;
    }
    let last_to_win = time_of_victory.iter().position_max().unwrap();
    println!("Last Number: {}", boards[last_to_win].victory_number);
    println!("Board Score: {}", boards[last_to_win].sum_unmarked());
    println!("Final Result: {}", boards[last_to_win].victory_number * boards[last_to_win].sum_unmarked());
}

fn task01(){
    let numbers = get_numbers();
    let mut boards = boards();

    for number in numbers{
        let mut victory = false;
        let mut winning_idx = 0;
        for board_idx in 0..boards.len(){
            boards[board_idx].update_board(number);
            victory = boards[board_idx].check_victory();
            if victory{
                winning_idx = board_idx;
                break;
            }
        }
        if victory {
            println!("Last Number: {}", number);
            println!("Board Score: {}", boards[winning_idx].sum_unmarked());
            println!("Final Result: {}", number*boards[winning_idx].sum_unmarked());
            break;
        }
    }
}

#[derive(Clone)]
struct Board{
    board: Vec<Vec<i32>>,
    board_status: Vec<Vec<bool>>,
    victory_number: i32
}

impl Board {

    pub fn update_board(&mut self, number: i32) {
        for line in 0..self.board.len(){
            let index = self.board[line].iter().positions(|&r| r == number);
            for idx in index{
                self.board_status[line][idx] = true;
            }
        }
    }

    pub fn check_victory(&self) -> bool {
        for line in 0..self.board_status.len(){
            let marked = self.board_status[line].iter().filter(|&n| *n == true).count();
            if marked == 5 {
                return true;
            }
        }
        for column in 0..self.board_status[0].len(){
            let mut marked = 0;
            for line in 0..self.board_status.len(){
                if self.board_status[line][column]{
                    marked += 1;
                }
                if marked == 5 {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn sum_unmarked(&self) -> i32{
        let mut sum = 0;
        for line in 0..self.board_status.len() {
            let unmarked = self.board_status[line].iter().positions(|&n| n == false);
            for idx in unmarked {
                sum += self.board[line][idx];
            }
        }
        return sum;
    }

}

fn get_numbers() -> Vec<i32>{

    let mut out_number: Vec<i32> = Vec::new();

    let filename = "./data/day04.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file").replace("\r", "");

    let data = contents.split("\n").collect::<Vec<&str>>()[0];
    let numbers = data.split(",").collect::<Vec<&str>>();
    for number in numbers{
        out_number.push(number.parse::<i32>().unwrap());
    }
    return out_number;
}

fn boards() -> Vec<Board>{

    let mut out_boards: Vec<Board> = Vec::new();

    let filename = "./data/day04.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file").replace("\r", "");

    let data = contents.split("\n").map(str::to_string).collect::<Vec<String>>();
    let sep = Regex::new(r"[ ]+").unwrap();

    for idx in (2..data.len()).step_by(6) {
        let mut board_buff: Vec<Vec<i32>> = Vec::new();

        for line_nr in 0..5{
            let mut line_buff: Vec<i32> = Vec::new();
            let data_buff = sep.split(&data[idx + line_nr].trim()).into_iter().collect::<Vec<&str>>();

            for entry in data_buff{
                line_buff.push(entry.parse::<i32>().unwrap());
            }

            board_buff.push(line_buff.clone());

        }

        out_boards.push(Board{board: board_buff, board_status: vec![vec![false; 5]; 5], victory_number: 0})

    }
    return out_boards;
}