use vstd::prelude::*;
verus! {
fn all_prefixes(s: Vec<char>) -> (result: Vec<Vec<char>>){
    let mut result: Vec<Vec<char>> = Vec::new();
    let mut prefix: Vec<char> = Vec::new();
    let mut i: usize = 0;
    while i < s.len()
    {
        let ch = s[i];
        prefix.push(ch);
        let snapshot = prefix.clone();
        result.push(snapshot);
        i += 1;
    }
    result
}
}
fn main() {}