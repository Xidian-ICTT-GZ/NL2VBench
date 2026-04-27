use vstd::prelude::*;
verus! {
fn solve(input: Vec<char>) -> (result: Vec<char>){
    let b = (input[0] != input[1]) || (input[1] != input[2]);
    let mut res: Vec<char> = Vec::new();
    if b {
        res.push('Y');
        res.push('e');
        res.push('s');
    } else {
        res.push('N');
        res.push('o');
    }
    res
}
}
fn main() {}