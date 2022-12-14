use std::collections::HashMap;
use std::fs;

fn main(){
    let rom = load_rom("./data/day24.txt");
    let mut previous: Vec<i64> = vec![9; 14];
    previous[13] = 10;
    let mut largest = previous.clone();
    loop {
        let new = decrement_input(&previous);
        if new == previous{
            break;
        }
        let mut processor = ProcessorState::new(&new);
        for instruction in &rom{
            processor.execute(instruction);
        }
        if *processor.registers.entry('z').or_insert(0) == 0 {
            largest = new.clone();
            break;
        }
        previous = new;
    }
    println!("{:?}", largest);
}

fn decrement_input(input: &Vec<i64>) -> Vec<i64> {
    let mut output = input.clone();
    for idx in (0..output.len()).rev(){
        if output[idx] == 1 {
            if idx == 0{
                break;
            }
            output[idx] = 9;
            continue;
        }
        output[idx] -= 1;
        break;
    }
    output
}

fn increment_input(input: &Vec<i64>) -> Vec<i64>{
    let mut output = input.clone();
    for idx in 0..output.len(){
        if output[idx] == 9 {
            if idx == output.len() - 1{
                break;
            }
            output[idx] = 1;
            continue;
        }
        output[idx] += 1;
        break;
    }
    output
}

struct ProcessorState{
    registers: HashMap<char, i64>,
    input: Vec<i64>,
    input_pointer: usize,
}

impl ProcessorState{
    pub fn new(input: &Vec<i64>) -> ProcessorState{
        let mut registers: HashMap<char, i64> = HashMap::new();
        registers.insert('w', 0);
        registers.insert('x', 0);
        registers.insert('y', 0);
        registers.insert('z', 0);
        ProcessorState{registers, input: input.clone(), input_pointer: 0}
    }

    pub fn execute(&mut self, instruction: &Instruction){
        match instruction.operation{
            Operation::Inp => {
                match instruction.args[0] {
                    Argument::Num(_) => {}
                    Argument::Var(v) => {
                        *self.registers.entry(v).or_insert(0) = self.input[self.input_pointer];
                        self.input_pointer += 1;
                    }
                }
            }
            Operation::Add => {
                let mut target_reg= 'w';
                match instruction.args[0] {
                    Argument::Num(_) => {}
                    Argument::Var(v) => {target_reg = v}
                }
                let mut num = 0;
                match instruction.args[1] {
                    Argument::Num(v) => {num = v}
                    Argument::Var(v) => {num = *self.registers.entry(v).or_insert(0)}
                }
                *self.registers.entry(target_reg).or_insert(0) += num;
            }
            Operation::Mul => {
                let mut target_reg= 'w';
                match instruction.args[0] {
                    Argument::Num(_) => {}
                    Argument::Var(v) => {target_reg = v}
                }
                let mut num = 0;
                match instruction.args[1] {
                    Argument::Num(v) => {num = v}
                    Argument::Var(v) => {num = *self.registers.entry(v).or_insert(0)}
                }
                *self.registers.entry(target_reg).or_insert(0) *= num;
            }
            Operation::Div => {
                let mut target_reg= 'w';
                match instruction.args[0] {
                    Argument::Num(_) => {}
                    Argument::Var(v) => {target_reg = v}
                }
                let mut num = 0;
                match instruction.args[1] {
                    Argument::Num(v) => {num = v}
                    Argument::Var(v) => {num = *self.registers.entry(v).or_insert(0)}
                }
                *self.registers.entry(target_reg).or_insert(0) /= num;
            }
            Operation::Mod => {
                let mut target_reg= 'w';
                match instruction.args[0] {
                    Argument::Num(_) => {}
                    Argument::Var(v) => {target_reg = v}
                }
                let mut num = 0;
                match instruction.args[1] {
                    Argument::Num(v) => {num = v}
                    Argument::Var(v) => {num = *self.registers.entry(v).or_insert(0)}
                }
                *self.registers.entry(target_reg).or_insert(0) %= num;
            }
            Operation::Eql => {
                let mut target_reg= 'w';
                match instruction.args[0] {
                    Argument::Num(_) => {}
                    Argument::Var(v) => {target_reg = v}
                }
                let mut num = 0;
                match instruction.args[1] {
                    Argument::Num(v) => {num = v}
                    Argument::Var(v) => {num = *self.registers.entry(v).or_insert(0)}
                }
                let cur = *self.registers.entry(target_reg).or_insert(0);
                if cur == num{
                    *self.registers.entry(target_reg).or_insert(0) = 1;
                }
                else{
                    *self.registers.entry(target_reg).or_insert(0) = 0;
                }
            }
        }
    }
}

enum Operation{
    Inp,
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

struct Instruction{
    operation: Operation,
    args: Vec<Argument>,
}

impl Instruction{
    pub fn new(instruction: &String, args: &[String]) -> Instruction{
        let op: Operation;
        match instruction.as_str(){
            "inp" => op = Operation::Inp,
            "add" => op = Operation::Add,
            "mul" => op = Operation::Mul,
            "div" => op = Operation::Div,
            "mod" => op = Operation::Mod,
            "eql" => op = Operation::Eql,
            _ => op = Operation::Inp,
        }
        let mut oargs = Vec::new();
        match args[0].as_str() {
            "w" => oargs.push(Argument::Var('w')),
            "x" => oargs.push(Argument::Var('x')),
            "y" => oargs.push(Argument::Var('y')),
            "z" => oargs.push(Argument::Var('z')),
            _ => ()
        }
        if args.len() > 1 {
            let number = args[1].parse::<i64>();
            match number {
                Ok(ok) => oargs.push(Argument::Num(ok)),
                Err(_) => {
                    match args[1].as_str() {
                        "w" => oargs.push(Argument::Var('w')),
                        "x" => oargs.push(Argument::Var('x')),
                        "y" => oargs.push(Argument::Var('y')),
                        "z" => oargs.push(Argument::Var('z')),
                        _ => ()
                    }
                }
            }
        }
        Instruction{operation: op, args: oargs}
    }
}

enum Argument{
    Num(i64),
    Var(char),
}

fn parse_instruction(instruction: &str) -> Instruction{
    let entries: Vec<String> = instruction.split(' ').map(str::to_string).collect();
    Instruction::new(&entries[0], &entries[1..])
}

fn load_rom(path: &str) -> Vec<Instruction> {
    let contents = fs::read_to_string(path)
        .expect("Something went wrong reading the file").replace('\r', "");
    let mut output = Vec::new();
    for line in contents.trim().lines() {
            output.push(parse_instruction(line))
    }
    output
}