use std::fs;
use std::collections::HashSet;
use std::cmp;

fn main() {
    let commands = get_commands();
    let mut points_on: HashSet<Point> = HashSet::new();
    for command in &commands{
        run_command(&mut points_on, *command);
    }
    let t1 = points_on.len();
    println!("Task 1: {}", t1);
    let mut cubes = Vec::new();
    let mut signs = Vec::new();
    for command in &commands{
        run_command_pt2(&mut cubes, &mut signs, *command);
    }
    println!("Task 2: {}", volume(&cubes, &signs));
}



fn run_command(points: &mut HashSet<Point>, command: Command){
    let mut command  = command;
    command.trim();
    for x in command.x_start .. command.x_stop{
        for y in command.y_start .. command.y_stop{
            for z in command.z_start .. command.z_stop{
                if command.status {
                    points.insert(Point{x, y, z});
                }
                else {
                    points.remove(&Point{x, y, z});
                }
            }
        }
    }
}

fn run_command_pt2(cubes: &mut Vec<Cube>, sign: &mut Vec<bool>, command: Command){
        for cube_idx in 0..cubes.len(){
            let intersect = cubes[cube_idx].intersection(command.cube());
            if let Some(intersection) = intersect {
                cubes.push(intersection);
                sign.push(!sign[cube_idx]);
            }
        }
        if command.status{
            cubes.push(command.cube());
            sign.push(command.status);
        }
}

fn volume(cubes: &[Cube], sign: &[bool]) -> i128{
    let mut ovolume = 0;
    for idx in 0..cubes.len(){
        let mut volume = cubes[idx].volume();
        if !sign[idx] {volume *= -1};
        ovolume += volume;
    }
    ovolume
}

fn get_commands() -> Vec<Command>{
    let filename = "./data/day22.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file").replace('\r', "");
    let lines: Vec<String> = contents.split('\n').map(str::to_string).collect();
    let mut commands: Vec<Command> = Vec::new();
    for line in lines.iter(){
        let buff: Vec<String> = line.split(' ').map(str::to_string).collect();
        let status = buff[0] == "on";
        let mut axis_lim: Vec<String>  = buff[1].split(',').map(str::to_string).collect();
        axis_lim.iter_mut().for_each(|s| s.replace_range(0..2, ""));
        let x_lim: Vec<i64> = axis_lim[0].split("..").map(|s| s.parse().unwrap()).collect();
        let y_lim: Vec<i64> = axis_lim[1].split("..").map(|s| s.parse().unwrap()).collect();
        let z_lim: Vec<i64> = axis_lim[2].split("..").map(|s| s.parse().unwrap()).collect();
        commands.push(Command::new(status, x_lim[0], x_lim[1] + 1, y_lim[0], y_lim[1] + 1, z_lim[0], z_lim[1] + 1));
    }
    commands
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
struct Command{
    status: bool,
    x_start: i64,
    x_stop: i64,
    y_start: i64,
    y_stop: i64,
    z_start: i64,
    z_stop: i64,
}

impl Command{
    pub fn new(status: bool, x_start: i64, x_stop: i64, y_start: i64, y_stop: i64, z_start: i64, z_stop: i64) -> Command{
        Command{status, x_start, x_stop, y_start, y_stop, z_start, z_stop}
    }

    pub fn trim(&mut self){
        self.x_start = if self.x_start < -50 {-50} else {self.x_start};
        self.x_stop = if self.x_stop > 50 {50} else {self.x_stop};
        self.y_start = if self.y_start < -50 {-50} else {self.y_start};
        self.y_stop = if self.y_stop > 50 {50} else {self.y_stop};
        self.z_start = if self.z_start < -50 {-50} else {self.z_start};
        self.z_stop = if self.z_stop > 50 {50} else {self.z_stop};
    }

    pub fn cube(&self) -> Cube{
        Cube{x_start: self.x_start, x_stop: self.x_stop,
            y_start: self.y_start, y_stop: self.y_stop,
            z_start: self.z_start, z_stop: self.z_stop}
    }

}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
struct Point{
    x: i64,
    y: i64,
    z: i64,
}

struct Cube{
    x_start: i64,
    x_stop: i64,
    y_start: i64,
    y_stop: i64,
    z_start: i64,
    z_stop: i64,
}

impl Cube {

    pub fn volume(&self) ->i128{
        ((self.x_stop - self.x_start) * (self.y_stop - self.y_start) * (self.z_stop - self.z_start)) as i128
    }

    pub fn intersection(&self, other: Cube) -> Option<Cube>{
            let x_start = cmp::max(self.x_start, other.x_start);
            let x_stop = cmp::min(self.x_stop, other.x_stop);
            let y_start = cmp::max(self.y_start, other.y_start);
            let y_stop = cmp::min(self.y_stop, other.y_stop);
            let z_start = cmp::max(self.z_start, other.z_start);
            let z_stop = cmp::min(self.z_stop, other.z_stop);
            if x_start > x_stop || y_start > y_stop || z_start > z_stop {
                None
            }
            else {
                Some(Cube{x_start, x_stop, y_start, y_stop, z_start, z_stop})
            }
        }
}