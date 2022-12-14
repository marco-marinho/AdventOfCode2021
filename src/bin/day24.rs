use std::fs;
use std::cmp;
use std::collections::HashMap;

fn main() {
    let params = load_params("./data/day24.txt");
    track(params);
}

fn track(params: Vec<(i64, i64, i64)>) {
    let mut zs: HashMap<i64, (i64, i64)> = HashMap::new();
    zs.insert(0, (0, 0));
    for (idx, parameter) in params.iter().enumerate() {
        let mut new_zs: HashMap<i64, (i64, i64)> = HashMap::new();
        for key in zs.keys() {
            let inp = zs.get(key).unwrap();
            let left = params.len() as i64 - idx as i64;
            if *key as i128 >= (26_i128.pow(left as u32)) as i128 {
                continue;
            }

            for w in 1..10 {
                let new_z = block(*parameter, *key, w);
                if parameter.0 == 1 || (parameter.0 == 26 && new_z < *key) {

                    let new_low = inp.0 * 10 + w;
                    let new_high = inp.1 * 10 + w;
                    if !new_zs.contains_key(&new_z){
                        new_zs.insert(new_z, (new_low, new_high));
                    }
                    else{
                        let (old_low, old_high) = *new_zs.get(&new_z).unwrap();
                        let new_low = cmp::min(old_low, new_low);
                        let new_high = cmp::max(old_high, new_high);
                        new_zs.insert(new_z, (new_low, new_high));
                    }
                }
            }
        }
        zs = new_zs.clone();
    }
    let (low, high) = zs.get(&0).unwrap();
    println!("Task 01: {}", high);
    println!("Task 02: {}", low);
}

fn block(params: (i64, i64, i64), z: i64, w: i64) -> i64 {
    if z % 26 + params.1 == w {
        z / params.0
    } else {
        z / params.0 * 26 + w + params.2
    }
}


fn load_params(path: &str) -> Vec<(i64, i64, i64)> {
    let contents = fs::read_to_string(path)
        .expect("Something went wrong reading the file").replace('\r', "");
    let lines: Vec<String> = contents.split('\n').map(str::to_string).collect();
    let mut output = Vec::new();
    for idx in (0..18 * 14).step_by(18) {
        let a = lines[idx + 4].split_whitespace()
            .next_back()
            .unwrap_or("0")
            .parse::<i64>()
            .unwrap();
        let b = lines[idx + 5].split_whitespace()
            .next_back()
            .unwrap_or("0")
            .parse::<i64>()
            .unwrap();
        let c = lines[idx + 15].split_whitespace()
            .next_back()
            .unwrap_or("0")
            .parse::<i64>()
            .unwrap();
        output.push((a, b, c));
    }
    output
}