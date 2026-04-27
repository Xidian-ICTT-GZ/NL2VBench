use vstd::prelude::*;
verus! {
fn exec_last_occurrence_position(cafes: &Vec<i8>, cafe: i8) -> (pos: usize){
    let mut i = cafes.len();
    while i > 0
    {
        i = i - 1;
        if cafes[i] == cafe {
            return i;
        }
    }
    0
}
fn solve(cafes: Vec<i8>) -> (mini: i8){
    let mut mini = cafes[0];
    let mut min_last_pos = exec_last_occurrence_position(&cafes, mini);
    let mut i: usize = 1;
    while i < cafes.len()
    {
        let current_cafe = cafes[i];
        let current_last_pos = exec_last_occurrence_position(&cafes, current_cafe);
        if current_last_pos < min_last_pos {
            mini = current_cafe;
            min_last_pos = current_last_pos;
        }
        i = i + 1;
    }
    mini
}
}
fn main() {}