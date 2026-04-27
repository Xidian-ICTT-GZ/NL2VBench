use vstd::prelude::*;
verus! {
fn solve(n: i8, s: Vec<char>) -> (result: i8){
    let mut current_value: i8 = 0;
    let mut max_value: i8 = 0;
    let mut i: usize = 0;
    while i < n as usize
    {
        if s[i] == 'I' {
            current_value = current_value + 1;
        } else {
            current_value = current_value - 1;
        }
        if current_value > max_value {
            max_value = current_value;
        }
        i = i + 1;
    }
    max_value
}
}
fn main() {}