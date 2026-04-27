use vstd::prelude::*;
verus! {
fn solve(date_str: Vec<char>) -> (result: Vec<char>){
    let mut result: Vec<char> = Vec::with_capacity(10);
    result.push('2');
    result.push('0');
    result.push('1');
    result.push('8');
    let mut i: usize = 4;
    while i < 10
    {
        result.push(date_str[i]);
        i = i + 1;
    }
    result
}
}
fn main() {}