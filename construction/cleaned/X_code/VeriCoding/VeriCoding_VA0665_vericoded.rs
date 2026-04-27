use vstd::prelude::*;
verus! {
fn solve(lines: Vec<Vec<char>>) -> (result: Vec<char>){
    let mut result = Vec::new();
    result.push(lines[0][0]);
    result.push(lines[1][1]);
    result.push(lines[2][2]);
    result.push('\n');
    result
}
}
fn main() {}