use vstd::prelude::*;
verus! {
fn solve(s: Vec<char>) -> (result: Vec<char>){
    let mut result = Vec::<char>::new();
    let mut i: usize = 0;
    while i < s.len()
    {
        let key = s[i];
        if key == 'B' {
            if result.len() > 0 {
                result.pop();
            }
        } else {
            result.push(key);
        }
        i = i + 1;
    }
    result
}
}
fn main() {}