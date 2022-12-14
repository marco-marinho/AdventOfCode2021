use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    task01();
    task02();
}

fn task02() {
    let data = get_strings_2();
    let inputs = data.0;
    let ouputs = data.1;
    let mut sum: u32 = 0;
    for entry in 0..inputs.len() {
        let decoder = gen_decoder(inputs[entry].clone());
        let output = ouputs[entry].clone();
        let value = get_value(output, decoder);
        sum = sum + value;
    }
    println!("{}", sum);
}

fn get_value(output: Vec<HashSet<char>>, decoder: HashMap<Vec<char>, u8>) -> u32 {
    let mut value: u32 = 0;
    let mut key: Vec<char> = output[0].clone().into_iter().collect();
    key.sort();
    value += *decoder.get(&key).unwrap() as u32 * 1000;
    let mut key: Vec<char> = output[1].clone().into_iter().collect();
    key.sort();
    value += *decoder.get(&key).unwrap() as u32 * 100;
    let mut key: Vec<char> = output[2].clone().into_iter().collect();
    key.sort();
    value += *decoder.get(&key).unwrap() as u32 * 10;
    let mut key: Vec<char> = output[3].clone().into_iter().collect();
    key.sort();
    value += *decoder.get(&key).unwrap() as u32;
    return value;
}

fn gen_decoder(input: Vec<HashSet<char>>) ->  HashMap<Vec<char>, u8>{
    let mut buff = input.clone();
    let mut demux: HashMap<Vec<char>, u8> = HashMap::new();
    let one = buff.iter().filter(|x| x.len() == 2).next().unwrap().clone();
    buff.retain(|x| *x != one);
    let mut key: Vec<char> = one.clone().into_iter().collect();
    key.sort();
    demux.insert(key, 1);


    let four = buff.iter().filter(|x| x.len() == 4).next().unwrap().clone();
    buff.retain(|x| *x != four);
    key = four.clone().into_iter().collect();
    key.sort();
    demux.insert(key, 4);

    let seven = buff.iter().filter(|x| x.len() == 3).next().unwrap().clone();
    buff.retain(|x| *x != seven);
    key = seven.clone().into_iter().collect();
    key.sort();
    demux.insert(key, 7);

    let eight = buff.iter().filter(|x| x.len() == 7).next().unwrap().clone();
    buff.retain(|x| *x != eight);
    key = eight.clone().into_iter().collect();
    key.sort();
    demux.insert(key, 8);

    let three = buff.iter().filter(|x| x.len() == 5 && (*x & &one).len() == 2).next().unwrap().clone();
    buff.retain(|x| *x != three);
    key = three.clone().into_iter().collect();
    key.sort();
    demux.insert(key, 3);

    let two = buff.iter().filter(|x| x.len() == 5 && (*x & &four).len() == 2).next().unwrap().clone();
    buff.retain(|x| *x != two);
    key = two.clone().into_iter().collect();
    key.sort();
    demux.insert(key, 2);

    let five = buff.iter().filter(|x| x.len() == 5).next().unwrap().clone();
    buff.retain(|x| *x != five);
    key = five.clone().into_iter().collect();
    key.sort();
    demux.insert(key, 5);

    let six = buff.iter().filter(|x| x.len() == 6 && (*x & &one).len() == 1).next().unwrap().clone();
    buff.retain(|x| *x != six);
    key = six.clone().into_iter().collect();
    key.sort();
    demux.insert(key, 6);

    let nine = buff.iter().filter(|x| x.len() == 6 && (*x & &four).len() == 4).next().unwrap().clone();
    buff.retain(|x| *x != nine);
    key = nine.clone().into_iter().collect();
    key.sort();
    demux.insert(key, 9);

    let zero = buff.iter().filter(|x| x.len() == 6).next().unwrap().clone();
    buff.retain(|x| *x != zero);
    key = zero.clone().into_iter().collect();
    key.sort();
    demux.insert(key, 0);

    return demux;
}

fn task01() {

    let input = get_strings();
    let target = [2, 4, 3, 7];
    let mut count = 0;
    for line in input {
        let buff: Vec<String> = line.trim().split(" ").map(str::to_string).collect();
        for entry in buff{
            if target.contains(&(entry.len() as i32)) {
                count+=1;
            }
        }
    }
    println!("{}", count);

}

fn get_strings() -> Vec<String>{
    let mut output: Vec<String> = Vec::new();

    let filename = "./data/day08.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file").replace("\r", "");

    let entries: Vec<String> = contents.split("\n").map(str::to_string).collect();

    for entry in &entries {
        let temp: Vec<String> = entry.split("|").map(str::to_string).collect();
        output.push(temp[1].clone());
    }
    return output;
}

fn get_strings_2() -> (Vec<Vec<HashSet<char>>>, Vec<Vec<HashSet<char>>>){

    let mut inputs: Vec<Vec<HashSet<char>>> = Vec::new();
    let mut outputs: Vec<Vec<HashSet<char>>> = Vec::new();

    let filename = "./data/day08.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file").replace("\r", "");

    let entries: Vec<String> = contents.split("\n").map(str::to_string).collect();

    for entry in &entries{
        let line: Vec<String> = entry.split("|").map(str::to_string).collect();
        let temp_input: Vec<String> = line[0].trim().split(" ").map(str::to_string).collect();
        let temp_output: Vec<String> = line[1].trim().split(" ").map(str::to_string).collect();
        let mut input_vec = Vec::new();
        for input in &temp_input {
            let mut buff_input = HashSet::new();
            for ch in input.chars().collect::<Vec<char>>() {
                buff_input.insert(ch);
            }
            input_vec.push(buff_input.clone());
        }
        inputs.push(input_vec);
        let mut output_vec = Vec::new();
        for output in &temp_output {
            let mut buff_output = HashSet::new();
            for ch in output.chars().collect::<Vec<char>>() {
                buff_output.insert(ch);
            }
            output_vec.push(buff_output.clone());
        }
        outputs.push(output_vec);
    }

    return (inputs, outputs);
}