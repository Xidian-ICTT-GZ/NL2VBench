use vstd::prelude::*;
verus! {
fn solve(n: i8, s: Vec<char>) -> (result: i8){
    let mut count: i8 = 0;
    let mut i: i8 = 1;
    while i < n
    {
        if s[i as usize] == s[(i - 1) as usize] {
            count = count + 1;
        }
        i = i + 1;
    }
    count
}
}
fn main() {}