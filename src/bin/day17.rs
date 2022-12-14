use std::fs;

fn main() {
    task01();
    task02();
}

fn task02(){
    let limits = get_limits();
    let mut num_velocities = 0;
    for x_speed in 0..1000 {
        for y_speed in -1000..1000 {
            let result = check_launch(&Point(x_speed,y_speed), &limits);
            if result.0{
                num_velocities += 1;
            }
        }
    }
    println!("{}", num_velocities);
}

fn task01(){
    let limits = get_limits();
    let mut max_y = 0;
    for x_speed in 0..1000 {
        for y_speed in 0..1000 {
            let result = check_launch(&Point(x_speed,y_speed), &limits);
            if result.0 && result.1 > max_y{
                max_y = result.1;
            }
        }
    }
    println!("{}", max_y);
}

fn check_launch(speed: &Point, limits: &Vec<Point>) -> (bool, i32){
    let mut position = Point(0, 0);
    let mut speed = (*speed).clone();
    let mut largest_y = i32::MIN;
    while position.0 <= limits[0].1 && position.1 >= limits[1].0 {
        position = Point(position.0 + speed.0, position.1 + speed.1);
        if position.1 > largest_y {largest_y = position.1}
        if check_if_contained(&position, &limits){
            return (true, largest_y);
        }
        speed = Point(if speed.0 > 0 {speed.0 - 1} else {0}, speed.1 - 1);
    }
    return (false, largest_y);
}

fn check_if_contained(position: &Point, limits: &Vec<Point>) -> bool{
    if position.0 >= limits[0].0 && position.0 <= limits[0].1 && position.1 <= limits[1].1 && position.1 >= limits[1].0 {
        return true;
    }
    return false;
}

fn get_limits() -> Vec<Point>{
    let filename = "./data/day17.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file").replace("\r", "");
    let filtered = contents.replace("target area: x=", "").replace(", y=", ";").replace("..", ",");
    let filtered_split: Vec<String> = filtered.split(";").map(str::to_string).collect();
    let mut output: Vec<Point> = Vec::new();
    for element in filtered_split {
        let limits: Vec<i32> = element.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
        let point = Point(limits[0], limits[1]);
        output.push(point);
    }
    return output;
}

#[derive(Debug, Clone)]
struct Point(i32, i32);