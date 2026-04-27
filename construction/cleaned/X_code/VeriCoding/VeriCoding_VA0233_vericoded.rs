use vstd::prelude::*;
verus! {
fn solve(position: Vec<char>) -> (moves: i8){
    let c0 = position[0];
    let c1 = position[1];
    let file_edge = c0 == 'a' || c0 == 'h';
    let rank_edge = c1 == '1' || c1 == '8';
    let moves: i8 = if file_edge && rank_edge {
        3i8
    } else if file_edge || rank_edge {
        5i8
    } else {
        8i8
    };
    moves
}
}
fn main() {}