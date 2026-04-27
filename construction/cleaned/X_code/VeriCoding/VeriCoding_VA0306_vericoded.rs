use vstd::prelude::*;
verus! {
fn copy_char_at(source: &Vec<char>, target: &mut Vec<char>, pos: usize){
    target.set(pos, source[pos]);
}
fn solve(t: Vec<char>) -> (result: Vec<char>){
    let mut result = Vec::with_capacity(t.len());
    let mut i = 0;
    while i < t.len()
    {
        result.push(t[i]);
        i += 1; 
    }
    result
}
}
fn main() {}