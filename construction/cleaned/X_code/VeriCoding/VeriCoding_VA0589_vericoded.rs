use vstd::prelude::*;
verus! {
fn compute_moves(a: i8, b: i8) -> (result: i8){
    if a == b {
        0
    } else if a < b {
        if (b - a) % 2 == 1 {
            1
        } else {
            2
        }
    } else { 
        if (a - b) % 2 == 0 {
            1
        } else {
            2
        }
    }
}
fn solve(input: Vec<(i8, i8)>) -> (result: Vec<i8>){
    let mut result: Vec<i8> = Vec::new();
    let mut i: usize = 0;
    while i < input.len()
    {
        let item = input[i];
        let a = item.0;
        let b = item.1;
        let moves = compute_moves(a, b);
        result.push(moves);
        i = i + 1;
    }
    result
}
}
fn main() {}