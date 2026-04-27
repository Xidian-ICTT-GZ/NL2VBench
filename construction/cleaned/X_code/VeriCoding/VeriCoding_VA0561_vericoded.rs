use vstd::prelude::*;
verus! {
fn solve(n: i32) -> (result: Vec<char>){
    let mut v: Vec<char> = Vec::new();
    v.push('A');
    v.push('B');
    if n < 1000 {
        v.push('C');
    } else {
        v.push('D');
    }
    v
}
}
fn main() {}