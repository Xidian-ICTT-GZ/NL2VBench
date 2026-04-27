use vstd::prelude::*;
verus! {
fn solve(s: Vec<char>) -> (result: Vec<char>){
    let mut result = Vec::new();
    let mut i = 0;
    while i < s.len()
    {
        if s[i] == ',' {
            result.push(' ');
        } else {
            result.push(s[i]);
        }
        i += 1;
    }
    result.push('\n');
    result
}
}
fn main() {}