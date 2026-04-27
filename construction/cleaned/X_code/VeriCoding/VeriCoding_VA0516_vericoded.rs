use vstd::prelude::*;
verus! {
fn solve(s: Vec<char>) -> (result: Vec<char>){
    let b = s[0] == s[1] || s[1] == s[2] || s[2] == s[3];
    let mut result: Vec<char> = Vec::new();
    if b {
        result.push('B');
        result.push('a');
        result.push('d');
    } else {
        result.push('G');
        result.push('o');
        result.push('o');
        result.push('d');
    }
    result
}
}
fn main() {}