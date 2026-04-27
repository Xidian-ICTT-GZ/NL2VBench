use vstd::prelude::*;
verus! {
fn solve(x: i32) -> (result: Vec<char>){
    let mut v: Vec<char> = Vec::new();
    v.push('A');
    if x < 1200 {
        v.push('B');
    } else {
        v.push('R');
    }
    v.push('C');
    v.push('\n');
    if x < 1200 {
    } else {
    }
    v
}
}
fn main() {}