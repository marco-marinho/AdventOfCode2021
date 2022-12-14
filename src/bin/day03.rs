use std::fs;
fn main() {

    task01();
    task02();

}

fn task02(){
    let commands = get_commands();
    let mut oxy: Vec<String> = commands.clone();
    let mut co2: Vec<String> = commands.clone();
    let mut idx = 0;

    while oxy.len() > 1 {
        let target: char;
        let freq = get_one_freq(&oxy);
        if freq[idx] >= 0.5{
            target = '1';
        }
        else{
            target = '0';
        }
        oxy.retain(|x| check_for_keeping(&x, target, idx as i32));
        idx+=1;
    }

    idx = 0;
    while co2.len() > 1 {
        let target: char;
        let freq = get_one_freq(&co2);
        if freq[idx] >= 0.5{
            target = '0';
        }
        else{
            target = '1';
        }
        co2.retain(|x| check_for_keeping(&x, target, idx as i32));
        idx+=1;
    }

    let o2 = isize::from_str_radix(&oxy[0], 2).unwrap();
    let co2 = isize::from_str_radix(&co2[0], 2).unwrap();
    println!("O2: {o2}, CO2: {co2}, Rating: {rating}", o2=o2, co2 = co2, rating = o2*co2);

}

fn check_for_keeping(command: &String, target: char, idx: i32) -> bool{

    let bit = command.chars().nth(idx as usize).unwrap();
    if bit == target{
        return true;
    }
    else{
        return false;
    }
}

fn get_one_freq(commands: &Vec<String>) -> Vec<f64>{
    let mut one_freq = vec![0.0; commands[0].len()];
    let num_lines = commands.len() as f64;
    for command in commands {
        let bits = command.chars().collect::<Vec<char>>();
        for bit_idx in 0..bits.len(){
            if bits[bit_idx] == '1'{
                one_freq[bit_idx]+= 1.0 / num_lines;
            }
        }
    }
    return one_freq;
}

fn get_commands() -> Vec<String>{
    let filename = "./data/day03.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file").replace("\r", "");

    let commands = contents.split("\n").map(str::to_string).collect();
    return commands;
}

fn task01(){
    let commands = get_commands();
    let num_bits = commands[0].len();
    let num_lines = commands.len() as f64;
    let mut one_count = vec![0.0; num_bits];

    for command in commands {
        let bits = command.chars().collect::<Vec<char>>();
        for bit_idx in 0..bits.len(){
            if bits[bit_idx] == '1'{
                one_count[bit_idx]+=1.0;
            }
        }
    }
    let mut gamma_rate = "".to_owned();
    let mut epsilon_rate= "".to_owned();
    for idx in 0..one_count.len(){
        let freq = one_count[idx] / num_lines;
        if freq > 0.5{
            gamma_rate.push_str("1");
            epsilon_rate.push_str("0");
        }
        else{
            gamma_rate.push_str("0");
            epsilon_rate.push_str("1");
        }
    }
    let gamma_val = isize::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon_val = isize::from_str_radix(&epsilon_rate, 2).unwrap();
    println!("Gamma: {gamma}, Episilon: {epsilon}, Power: {power}", gamma=gamma_val, epsilon = epsilon_val, power = gamma_val*epsilon_val);
}