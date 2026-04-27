use vstd::prelude::*;
verus! {
fn solve(s: Vec<char>) -> (result: Vec<char>){
    if s[0] != s[1] && s[1] != s[2] && s[0] != s[2] {
        vec!['Y', 'e', 's', '\n']
    } else {
        vec!['N', 'o', '\n']
    }
}
}
fn main() {}