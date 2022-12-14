use std::{
    fs,
};

fn main(){
    let (pods, depth) = get_pods("./data/day23.txt");
    let mut ans: Vec<i32> = Vec::new();
    dfs(pods, depth, 0, &mut ans);
    let min_value = ans.iter().min().unwrap();
    println!("Task 1: {}", min_value);
    let (pods, depth) = get_pods("./data/day23_2.txt");
    let mut ans: Vec<i32> = Vec::new();
    dfs(pods, depth, 0, &mut ans);
    let min_value = ans.iter().min().unwrap();
    println!("Task 2: {}", min_value);
}

fn dfs(pods: Vec<Amphipod>, depth: i32, cost: i32, ans: &mut Vec<i32>){
    if is_finished(&pods){
        ans.push(cost);
        return;
    }
    let pods_orig = pods.clone();
    for idx in 0..pods_orig.len(){
        let mut pods = pods_orig.clone();
        if pods[idx].in_target_room() {
            continue;
        }
        // Check if pod can move into a room, and move to the room
        if pods[idx].can_enter_room(&pods){
            let pos_in_room = pods[idx].room_position(&pods, depth);
            let path_to_room = pods[idx].path_to_point(pos_in_room);
            if is_path_clear(&path_to_room, &pods){
                pods[idx].position = pos_in_room;
                let cost_added = pods[idx].move_cost(&path_to_room);
                dfs(pods.clone(), depth, cost + cost_added, ans);
                // If a move to a room is possible, it is always optimal
                return;
            }
        }
    }
    for idx in 0..pods_orig.len(){
        // If the pod is already in the hall it can only move to room
        // If the pod is in target room, it might need to leave the room to let another pod leave
        if pods_orig[idx].in_hall() || !pods[idx].should_leave_room(&pods) {
            continue
        }
        let hall_len = 11;
        for hall_pos in 0..hall_len{
            let mut pods = pods_orig.clone();
            let target_post = (0, hall_pos);
            // Pod cannot stand in entryway
            if is_entryway(target_post) {continue}
            let path = pods[idx].path_to_point(target_post);
            // Move to position if path is clear and recurse
            if is_path_clear(&path, &pods){
                let cost_added = pods[idx].move_cost(&path);
                pods[idx].position = target_post;
                dfs(pods.clone(), depth, cost + cost_added, ans);
            }
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
enum Species{
    A,
    B,
    C,
    D,
}

impl Species{
    pub fn cost(&self) -> i32{
        match self {
            Species::A => 1,
            Species::B => 10,
            Species::C => 100,
            Species::D => 1000,
        }
    }

    pub fn target_room(&self) -> i32{
        match self {
            Species::A => 2,
            Species::B => 4,
            Species::C => 6,
            Species::D => 8,
        }
    }

    pub fn as_char(&self) -> char{
        match self {
            Species::A => 'A',
            Species::B => 'B',
            Species::C => 'C',
            Species::D => 'D',
        }
    }

}

#[derive(Clone, Debug)]
struct Amphipod{
    specie: Species,
    position: (i32, i32),
}

impl Amphipod{
    pub fn new(specie: char, position: (i32, i32)) -> Amphipod{
        let mut nspecie: Species = Species::A;
        match specie {
            'A' => nspecie = Species::A,
            'B' => nspecie = Species::B,
            'C' => nspecie = Species::C,
            'D' => nspecie = Species::D,
            _ => {}
        }
        Amphipod{specie: nspecie, position}
    }

    pub fn in_target_room(&self) -> bool{
        self.position.0 < 0 && self.specie.target_room() == self.position.1
    }

    pub fn in_hall(&self) -> bool{
        self.position.0 == 0
    }

    pub fn can_enter_room(&self, others: &Vec<Amphipod>) -> bool{
        let target_room = self.specie.target_room();
        for other in others{
            if other.position.0 < 0 && other.position.1 == target_room && other.specie != self.specie {
                return false
            }
        }
        true
    }

    pub fn room_position(&self, others: &Vec<Amphipod>, depth: i32) -> (i32, i32){
        let target_room = self.specie.target_room();
        let mut shallowest = depth - 1;
        for other in others{
            if other.position.1 == target_room && other.position.0 > shallowest {
                shallowest = other.position.0;
            }
        }
        (shallowest + 1, target_room)
    }

    pub fn path_to_point(&self, point: (i32, i32)) -> Vec<(i32,i32)>{
        let mut path = Vec::new();
        // If pod is in a room, move to hallway first.
        if self.position.0 < 0 {
            for row in self.position.0 .. 0{
                path.push((row, self.position.1))
            }
        }
        // Traverse hallway.
        if self.position.1 < point.1{
            for col in self.position.1 .. point.1 + 1{
                path.push((0, col));
            }
        }
        else {
            for col in (point.1 .. self.position.1 + 1).rev(){
                path.push((0, col))
            }
        }
        // If final destination is room, enter it.
        for row in 1 .. -(point.0)+1{
            path.push((-row, point.1))
        }
        path.remove(0);
        path
    }

    pub fn move_cost(&self, path: &[(i32,i32)]) -> i32{
        path.len() as i32 * self.specie.cost()
    }

    pub fn should_leave_room(&self, pods: &[Amphipod]) -> bool{
        if !self.in_target_room(){
            return true;
        }
        if self.in_target_room(){
            let depth = self.position.0;
            let room = self.position.1;
            for pod in pods{
                if pod.position.1 == room && pod.position.0 < depth && pod.specie != self.specie{
                    return true;
                }
            }
        }
        false
    }

}

fn is_finished(pods: &[Amphipod]) -> bool{
    for pod in pods{
        if !pod.in_target_room(){
            return false
        }
    }
    true
}

fn is_entryway(point: (i32, i32)) -> bool{
    point.0 == 0 && (point.1 == 2 || point.1 == 4 || point.1 == 6 || point.1 == 8)
}

fn is_path_clear(path: &[(i32, i32)], pods: &[Amphipod]) -> bool{
    for point in path {
        for pod in pods{
            if pod.position == *point {
                return false;
            }
        }
    }
    true
}

fn print_status(pods: &[Amphipod]){
    let mut map: Vec<Vec<char>> = vec![vec![' '; 11]; 7];
    for pod in pods{
        let coods = pod.position;
        map[-coods.0 as usize][coods.1 as usize] = pod.specie.as_char();
    }
    for line in map{
        println!("{}", String::from_iter(line));
    }
}


fn get_pods(path: &str) -> (Vec<Amphipod>, i32) {
    let contents = fs::read_to_string(path)
        .expect("Something went wrong reading the file").replace('\r', "");
    let mut output = Vec::new();
    for (row, line) in contents.trim().lines().enumerate() {
        for (col, entry) in line.chars().enumerate() {
            match entry {
                'A'|'B'|'C'|'D' => output.push(Amphipod::new(entry, (-((row - 1) as i32), (col - 1) as i32))),
                _ => {},
            }
        }
    }
    let depth = contents.lines().into_iter().count();
    (output, -(depth as i32 - 3))
}
