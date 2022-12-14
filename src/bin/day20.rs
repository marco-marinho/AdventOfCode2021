use std::{fs};

fn main() {
    let (enhancements, image) = get_scanners();
    let nimage = increase_frame(&image, false, 55);
    print_image(&nimage);
    let nimage = enhance(&nimage, &enhancements, false, 2);
    print_image(&nimage);
    let num_lit = count_lit_pixels(&nimage);
    println!("Task 01: {}", num_lit);
    let nimage = enhance(&nimage, &enhancements, false, 48);
    print_image(&nimage);
    let num_lit = count_lit_pixels(&nimage);
    println!("Task 02: {}", num_lit);
}

fn enhance(image: &[Vec<bool>], enhancements: &[bool], default: bool, times: u64) -> Vec<Vec<bool>>{
    let mut default = default;
    let mut image = image.to_vec();
    for _ in 0 .. times {
        image = enhance_image(&image, enhancements, default);
        default = !default;
    }
    image
}

fn count_lit_pixels(image: &[Vec<bool>]) -> u64{
    let nrows = image.len();
    let ncols = image[0].len();
    let mut output = 0;
    for i_row in 0 .. nrows{
        for i_col in 0 .. ncols {
            if image[i_row][i_col] { output += 1; }
        }
    }
    output
}

fn enhance_image(image: &[Vec<bool>], enhancements: &[bool], default: bool) -> Vec<Vec<bool>>{
    let nrows = image.len();
    let ncols = image[0].len();
    let mut nimage = image.to_vec();
    for i_row in 0 .. nrows{
        for i_col in 0 .. ncols {
            let pixel_val = get_pixel_value(image, i_row, i_col, default);
            let pixel = enhancements[pixel_val];
            nimage[i_row][i_col] = pixel;
        }
    }
    nimage
}

fn print_image(image: &[Vec<bool>]){
    for row in image{
        let buff: String = row.iter().map(|x| if !x {'.'} else {'#'}).collect();
        println!("{}", buff);
    }
    println!();
    println!();
}

fn get_pixel_value(image: &[Vec<bool>], row: usize, col: usize, default: bool) -> usize{
    let nrows = image.len() as i32;
    let ncols = image[0].len() as i32;
    let mut num = 0;
    let mut offset = 8;
    let row = row as i32;
    let col = col as i32;
    for i_row in row-1..row+2{
        for i_col in col-1..col+2{
            if i_row < 0 || i_col < 0 || i_row >= nrows || i_col >= ncols {
                num |= (default as usize) << offset;
            }
            else if image[i_row as usize][i_col  as usize]{
                num |= 1 << offset;
            }
            offset -= 1;
        }
    }
    num
}

fn increase_frame(image: &Vec<Vec<bool>>, value: bool, size: usize) -> Vec<Vec<bool>>{
    let ncols = image[0].len();
    let mut new_image: Vec<Vec<bool>> = Vec::new();
    let new_row = vec![value; ncols + (size * 2)];
    for _ in 0..size{
        new_image.push(new_row.clone());
    }
    for row in image{
        let mut buff = vec![value; size*2];
        buff.splice(size..size, row.clone());
        new_image.push(buff);
    }
    for _ in 0..size{
        new_image.push(new_row.clone());
    }
    new_image
}

fn get_scanners() -> (Vec<bool>, Vec<Vec<bool>>){
    let filename = "./data/day20.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file").replace('\r', "");
    let lines: Vec<String> = contents.split('\n').map(str::to_string).collect();
    let mut image: Vec<Vec<bool>> = Vec::new();
    let enhancement: Vec<bool> = lines[0].chars().into_iter().map(|x| x=='#').collect();
    for line in lines.iter().skip(1){
        let buff: Vec<bool> = line.chars().into_iter().map(|x| x=='#').collect();
        if !buff.is_empty() {image.push(buff)}
    }
    (enhancement, image)
}