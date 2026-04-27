use vstd::prelude::*;
verus! {
fn extract_rear_chars(s: &Vec<Vec<char>>) -> (result: Vec<char>){
    let mut result: Vec<char> = Vec::new();
    let mut i: usize = 0;
    while i < s.len()
    {
        result.push(s[i][s[i].len() - 1]);
        i = i + 1;
    }
    result
}
}
fn main() {}