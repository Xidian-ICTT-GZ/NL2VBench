use vstd::prelude::*;
verus! {
fn solve(s: &Vec<char>) -> (result: Vec<char>){
    let mut result = s.clone();
    if s[s.len() - 1] == 's' {
        result.push('e');
        result.push('s');
    } else {
        result.push('s');
    }
    result
}
}
fn main() {}