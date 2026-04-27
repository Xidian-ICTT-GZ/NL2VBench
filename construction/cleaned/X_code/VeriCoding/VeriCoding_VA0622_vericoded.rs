use vstd::prelude::*;
verus! {
fn build_result(ch: char) -> (result: Vec<char>){
    let mut v = Vec::new();
    v.push('A');
    v.push(ch);
    v.push('C');
    v.push('\n');
    v
}
fn solve(input: Vec<char>) -> (result: Vec<char>){
    let idx: usize = 8usize;
    let ch = input[idx];
    let result = build_result(ch);
    result
}
}
fn main() {}