use vstd::prelude::*;
verus! {
fn solve(a: i8, b: i8, s: Vec<char>) -> (result: Vec<char>){
    let mut dash_count: usize = 0;
    let mut dash_pos: usize = 0;
    let mut i: usize = 0;
    while i < s.len()
    {
        let c = s[i];
        if c == '-' {
            if dash_count == 0 {
                dash_count = 1;
                dash_pos = i;
            } else {
                return vec!['N', 'o'];
            }
        }
        i = i + 1;
    }
    if dash_count == 1 && dash_pos == a as usize {
        vec!['Y', 'e', 's']
    } else {
        vec!['N', 'o']
    }
}
}
fn main() {}