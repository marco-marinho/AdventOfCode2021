use ndarray::{Array2, s, ArrayView};
use std::fs;

fn main() {
    task01();
    task02();
}

fn task02(){
    let mut grid: Array2<i32> = Array2::<i32>::zeros((1000, 1000));
    let points = get_points();
    for pair in &points{
        let slice_bounds = slice_from_points(&pair[0], &pair[1]);
        if check_hor_ver(&pair[0], &pair[1]){
            let mut slice = grid.slice_mut(s![slice_bounds[0]..slice_bounds[1], slice_bounds[2]..slice_bounds[3]]);
            slice += &ArrayView::from(&[1]);
        }
        else{
            let size_diag = (slice_bounds[1])-slice_bounds[0];
            let y_range = (slice_bounds[3])-slice_bounds[2];
            if size_diag/y_range == 1 {
                let signs = diag_signs_from_points(&pair[0], &pair[1]);
                for offset in 0..size_diag {
                    grid[[(pair[0].x + signs[0]*offset) as usize, (pair[0].y + signs[1]*offset) as usize]] += 1;
                }
            }
        }
    }
    let danger = grid.iter().filter(|&n| *n >1).count();
    println!("Task 2 results: {}", danger);
}

fn task01(){
    let mut grid: Array2<i32> = Array2::<i32>::zeros((1000, 1000));
    let points = get_points();
    for pair in &points{
        if check_hor_ver(&pair[0], &pair[1]){
            let slice_bounds = slice_from_points(&pair[0], &pair[1]);
            let mut slice = grid.slice_mut(s![slice_bounds[0]..slice_bounds[1], slice_bounds[2]..slice_bounds[3]]);
            slice += &ArrayView::from(&[1]);
        }
    }
    let danger = grid.iter().filter(|&n| *n >1).count();
    println!("Task 1 results: {}", danger);
}

fn check_hor_ver(point1: &Point, point2: &Point) -> bool{
    if point1.x == point2.x || point1.y == point2.y {
        return true;
    }
    return false;
}

fn diag_signs_from_points(point1: &Point, point2: &Point) -> Vec<i32> {
    let signx;
    let signy;
    if point1.x < point2.x {
        signx = 1;
    }
    else{
        signx = -1;
    }
    if point1.y < point2.y {
        signy = 1;
    }
    else{
        signy = -1;
    }
    return Vec::from([signx, signy]);
}

fn slice_from_points(point1: &Point, point2: &Point) -> Vec<i32> {
    let xstart;
    let xend;
    let ystart;
    let yend;
    if point1.x < point2.x {
        xstart = point1.x;
        xend = point2.x + 1;
    }
    else{
        xstart = point2.x;
        xend = point1.x + 1;
    }
    if point1.y < point2.y {
        ystart = point1.y;
        yend = point2.y + 1;
    }
    else{
        ystart = point2.y;
        yend = point1.y + 1;
    }
    return Vec::from([xstart, xend, ystart, yend]);
}

struct Point{
    x: i32,
    y: i32
}

impl Point {
    pub fn from_string(input: &String) -> Point{
        let numbers: Vec<String> = input.split(",").map(str::to_string).collect();
        let x = numbers[0].parse::<i32>().unwrap();
        let y = numbers[1].parse::<i32>().unwrap();
        return Point{x, y};
    }
}

fn get_points() -> Vec<Vec<Point>>{

    let mut output: Vec<Vec<Point>> = Vec::new();

    let filename = "./data/day05.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file").replace("\r", "");

    let commands: Vec<String> = contents.split("\n").map(str::to_string).collect();

    for command in &commands{
        let points: Vec<String>= command.split(" -> ").map(str::to_string).collect();
        let mut buff: Vec<Point> = Vec::new();
        for point in &points{
            buff.push(Point::from_string(point));
        }
        output.push(buff);
    }
    return output;
}