use std::{fs};
use std::collections::{HashMap, HashSet};
use std::ops;

fn main() {
    let mut data = get_scanners();
    let mut points: HashSet<Point> = HashSet::new();
    let mut solved = vec![data[0].clone()];
    data.remove(0);
    while data.len() > 0{
        solve_one(&mut data, &mut solved);
    }
    for solution in &solved{
        for point_idx in 0..solution.points.len(){
            points.insert(solution.points[point_idx]);
        }
    }
    println!("Task 01: {}", points.len());
    let mut max_distance = 0;
    for first in 0..solved.len()-1{
        for second in first+1 ..solved.len(){
            let dist = manhatan_distance(&solved[first].coordinate, &solved[second].coordinate);
            if dist > max_distance {max_distance = dist;}
        }
    }
    println!("Task 02: {}", max_distance);
}

fn manhatan_distance(point_1: &Point, point_2: &Point) -> i64{
    (point_1.x - point_2.x).abs() + (point_1.y - point_2.y).abs() + (point_1.z - point_2.z).abs()
}

fn solve_one(data: &mut Vec<Scanner>, solved: &mut Vec<Scanner>) {
    for solution_idx in 0..solved.len() {
        let solution = &solved[solution_idx];
        for scanner_idx in 0..data.len() {
            let scanner = &mut data[scanner_idx];
            let (intersect_1, intersect_2) = solution.find_intersection(scanner);
            if intersect_1.len() >= 12 && intersect_2.len() >= 12 {
                scanner.all_distances();
                let (pair_1, pair_2) = find_one_intersection(&solution, &scanner);
                let (rotation, translation) = find_rotation(&pair_1, &pair_2);
                data[scanner_idx].update(&rotation, translation);
                data[scanner_idx].coordinate = translation;
                solved.push(data[scanner_idx].clone());
                data.remove(scanner_idx);
                return;
            }
        }
    }

}

fn find_rotation(points_1: &Vec<Point>, points_2: &Vec<Point>) -> (Vec<Vec<i64>>, Point) {
    let mut output_r: Vec<Vec<i64>> = Vec::new();
    let mut output_t = Point { x: 0, y: 0, z: 0 };
    for rotation in get_all_rotations() {
        let p1_1 = points_1[0].clone();
        let p1_2 = points_1[1].clone();
        let p2_1 = rotate(&points_2[0], &rotation);
        let p2_2 = rotate(&points_2[1], &rotation);
        if p1_1.clone() - p2_1.clone() == p1_2.clone() - p2_2.clone() {
            output_r = rotation.clone();
            output_t = p1_1.clone() - p2_1.clone();
            break;
        }

        if p1_1.clone() - p2_2.clone() == p1_2.clone() - p2_1.clone() {
            output_r = rotation.clone();
            output_t = p1_1.clone() - p2_2.clone();
            break;
        }
    }
    (output_r, output_t)
}

fn rotate(point: &Point, rotation: &Vec<Vec<i64>>) -> Point {
    let x = point.x * rotation[0][0] + point.y * rotation[0][1] + point.z * rotation[0][2];
    let y = point.x * rotation[1][0] + point.y * rotation[1][1] + point.z * rotation[1][2];
    let z = point.x * rotation[2][0] + point.y * rotation[2][1] + point.z * rotation[2][2];
    Point { x, y, z }
}

fn get_all_rotations() -> Vec<Vec<Vec<i64>>> {
    let signs = vec![1, -1];
    let shifts: Vec<usize> = vec![0, 1, 2];
    let orig = vec![1, 0, 0];
    let mut r_dims: Vec<Vec<i64>> = Vec::new();
    let mut rs: Vec<Vec<Vec<i64>>> = Vec::new();
    for r_shift in &shifts {
        let mut r_dim = orig.clone();
        r_dim.rotate_right(*r_shift);
        for sign in &signs {
            r_dim.iter_mut().for_each(|x| *x *= sign);
            r_dims.push(r_dim.clone());
        }
    }
    for x_idx in 0..r_dims.len() {
        for y_idx in 0..r_dims.len() {
            for z_idx in 0..r_dims.len() {
                let temp_x = r_dims[x_idx].clone();
                let temp_y = r_dims[y_idx].clone();
                let temp_z = r_dims[z_idx].clone();
                if temp_x[0].abs() + temp_y[0].abs() + temp_z[0].abs() != 1 { continue; }
                if temp_x[1].abs() + temp_y[1].abs() + temp_z[1].abs() != 1 { continue; }
                if temp_x[2].abs() + temp_y[2].abs() + temp_z[2].abs() != 1 { continue; }
                let det = determinant(&temp_x, &temp_y, &temp_z);
                if det != 1 { continue; }
                rs.push(vec![temp_x, temp_y, temp_z]);
            }
        }
    }
    rs
}

fn determinant(row_1: &Vec<i64>, row_2: &Vec<i64>, row_3: &Vec<i64>) -> i64 {
    let a = row_1[0] * (row_2[1] * row_3[2] - row_2[2] * row_3[1]);
    let b = row_1[1] * (row_2[0] * row_3[2] - row_2[2] * row_3[0]);
    let c = row_1[2] * (row_2[0] * row_3[1] - row_2[1] * row_3[0]);
    a - b + c
}


#[derive(Debug, Clone)]
struct Scanner {
    idx: i64,
    points: Vec<Point>,
    distances: HashMap<i64, (Point, Point)>,
    coordinate: Point,
}


#[derive(Debug, Clone, Hash, Copy)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl ops::Sub<Point> for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl Eq for Point {}

impl Scanner {
    pub fn new(points: Vec<Point>, idx: i64) -> Scanner {
        let mut output = Scanner {
            idx,
            points,
            distances: HashMap::new(),
            coordinate: Point { x: 0, y: 0, z: 0 },
        };
        output.all_distances();
        output
    }


    pub fn find_intersection(&self, other: &Scanner) -> (Vec<Point>, Vec<Point>) {
        let mut output_self = Vec::new();
        let mut output_other = Vec::new();
        for key_1 in self.distances.keys() {
            for key_2 in other.distances.keys() {
                if key_1 == key_2 {
                    let (o1, o2) = self.distances.get(key_1).unwrap().clone();
                    if !output_self.contains(&o1) && !output_self.contains(&o2) {
                        output_self.push(o1);
                        output_self.push(o2);
                    };
                    let (o1, o2) = other.distances.get(key_1).unwrap().clone();
                    if !output_other.contains(&o1) && !output_other.contains(&o2)
                    {
                        output_other.push(o1);
                        output_other.push(o2);
                    };
                }
            }
        }
        (output_self, output_other)
    }

    pub fn all_distances(&mut self) {
        self.distances.clear();
        for first in 0..self.points.len() - 1 {
            for second in first + 1..self.points.len() {
                let point_1 = &self.points[first];
                let point_2 = &self.points[second];
                let dist = get_distante(point_1, point_2);
                self.distances.insert(dist, (point_1.clone(), point_2.clone()));
            }
        }
    }

    pub fn update(&mut self, rotation: &Vec<Vec<i64>>, translation: Point) {
        let mut new_points: Vec<Point> = Vec::new();
        for point in &self.points {
            let rotated = rotate(&point, rotation);
            let translated = rotated + translation;
            new_points.push(translated);
        }
        self.points = new_points;
        self.all_distances();
    }
}

fn find_one_intersection(first: &Scanner, second: &Scanner) -> (Vec<Point>, Vec<Point>) {
    let mut output_self = Vec::new();
    let mut output_other = Vec::new();
    for key_1 in first.distances.keys() {
        for key_2 in second.distances.keys() {
            if key_1 == key_2 {
                let (o1, o2) = first.distances.get(key_1).unwrap().clone();
                output_self.push(o1);
                output_self.push(o2);
                let (o1, o2) = second.distances.get(key_1).unwrap().clone();
                output_other.push(o1);
                output_other.push(o2);
                return (output_self, output_other);
            }
        }
    }
    (output_self, output_other)
}

fn get_distante(point_1: &Point, point_2: &Point) -> i64 {
    (point_1.x - point_2.x).pow(2) + (point_1.y - point_2.y).pow(2) + (point_1.z - point_2.z).pow(2)
}


fn get_scanners() -> Vec<Scanner> {
    let filename = "./data/day19.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file").replace("\r", "");
    let lines: Vec<String> = contents.split("\n").map(str::to_string).collect();
    let mut output: Vec<Scanner> = Vec::new();
    let mut points = Vec::new();
    let mut idx = 0;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        if line.contains("scanner") {
            if points.len() > 0 {
                output.push(Scanner::new(points.clone(), idx));
                idx += 1;
            }
            points.clear();
        } else {
            let temp: Vec<i64> = line.split(",").map(|x| x.parse::<i64>().unwrap()).collect();
            points.push(Point { x: temp[0], y: temp[1], z: temp[2] })
        }
    }
    output.push(Scanner::new(points.clone(), idx));
    return output;
}