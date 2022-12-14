use std::collections::HashMap;

fn main(){
    let p1 = get_turns_deterministic(5 , 1);
    let p2 = get_turns_deterministic(10, 4);
    let val = calc_t1(&p1, &p2);
    println!("Task 1: {}", val);
    let (p1, p2) = get_turns_dirac(5, 10);
    let val = if p1 > p2 {p1} else {p2};
    println!("Task 2: {}", val);
}

fn calc_t1(p1: &[i64], p2: &[i64]) -> i64{
    let len1 = p1.len();
    let len2 = p2.len();
    if len1 < len2 {
        let lossing_score = p2[len1 - 2];
        let rolls = ((len1 * 2) - 1) * 3;
        lossing_score * rolls as i64
    }
    else {
        let lossing_score = p1[len2 - 1];
        let rolls = len2 * 2 * 3;
        lossing_score * rolls as i64
    }

}

fn get_turns_deterministic(start_pos: i64, start_die: i64) -> Vec<i64>{
    let mut output = Vec::new();
    let mut score = 0;
    let mut die = start_die - 6;
    let mut cur_pos = start_pos;
    while score < 1000 {
        die += 6;
        let steps = (die * 3) + 3;
        cur_pos = (cur_pos + steps) % 10;
        if cur_pos == 0 {cur_pos = 10;}
        score += cur_pos;
        output.push(score);
    }
    output
}

fn get_possibilities() -> HashMap<i64, i64>{
    let possibilities = vec![1,2,3];
    let mut scores: HashMap<i64, i64> = HashMap::new();
    for idx_1 in 0..possibilities.len(){
        for idx_2 in  0..possibilities.len(){
            for idx_3 in 0..possibilities.len(){
                let sum = possibilities[idx_1] + possibilities[idx_2] + possibilities[idx_3];
                *scores.entry(sum).or_insert(0) += 1;
            }
        }
    }
    scores
}

fn get_turns_dirac(start_pos_p1: i64, start_pos_p2: i64) -> (u64, u64){
    let mut finishes_p1: u64 = 0;
    let mut finishes_p2: u64 = 0;
    let possibilities = get_possibilities();
    let mut game_state: HashMap<GameState, u64> = HashMap::new();
    game_state.insert(GameState{pos_p1: start_pos_p1, score_p1: 0, pos_p2: start_pos_p2, score_p2: 0}, 1);
    while !game_state.is_empty() {
        let mut new_state: HashMap<GameState, u64> = HashMap::new();
        for (state, occurences) in &game_state{
            for (steps, occurences_die) in &possibilities{
                let mut new_pos = (state.pos_p1 + steps) % 10;
                if new_pos == 0 {new_pos = 10;}
                let new_score = state.score_p1 + new_pos;
                if new_score >= 21 {finishes_p1 += occurences * *occurences_die as u64}
                else{
                    let state_buff = GameState{pos_p1: new_pos, score_p1: new_score, pos_p2: state.pos_p2, score_p2: state.score_p2};
                    *new_state.entry(state_buff).or_insert(0) += occurences * *occurences_die as u64;
                }
            }
        }
        let mut new_state2: HashMap<GameState, u64> = HashMap::new();
        for (state, occurences) in &new_state{
            for (steps, occurences_die) in &possibilities{
                let mut new_pos = (state.pos_p2 + steps) % 10;
                if new_pos == 0 {new_pos = 10;}
                let new_score = state.score_p2 + new_pos;
                if new_score >= 21 {finishes_p2 += occurences * *occurences_die as u64}
                else{
                    let state_buff = GameState{ pos_p1: state.pos_p1, score_p1: state.score_p1, pos_p2: new_pos, score_p2: new_score};
                    *new_state2.entry(state_buff).or_insert(0) += occurences * *occurences_die as u64;
                }
            }
        }
        game_state = new_state2.clone();
    }
    (finishes_p1, finishes_p2)
}

#[derive(Debug, Clone, Hash, Copy, Eq, PartialEq)]
struct GameState{
    pos_p1: i64,
    score_p1: i64,
    pos_p2: i64,
    score_p2: i64
}