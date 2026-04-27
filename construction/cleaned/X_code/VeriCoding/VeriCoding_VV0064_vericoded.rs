use vstd::prelude::*;
verus! {
fn reverse_string(s: &Vec<char>) -> (result: Vec<char>){
    let n: usize = s.len();
    let mut out: Vec<char> = Vec::new();
    let mut i: usize = 0;
    while i < n
    {
        let j: usize = n - i - 1;
        let ch = s[j];
        out.push(ch);
        i += 1;
    }
    out
}
}
fn main() {}