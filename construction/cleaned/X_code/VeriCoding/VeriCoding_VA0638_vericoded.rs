use vstd::prelude::*;
verus! {
fn cmp_result(x: char, y: char) -> (result: Vec<char>){
    if x < y {
        vec!['<', '\n']
    } else if x > y {
        vec!['>', '\n']
    } else {
        vec!['=', '\n']
    }
}
fn solve(stdin_input: Vec<char>) -> (result: Vec<char>){
    let n = stdin_input.len();
    let x = stdin_input[0];
    let y = stdin_input[2];
    let result_vec = cmp_result(x, y);
    result_vec
}
}
fn main() {}