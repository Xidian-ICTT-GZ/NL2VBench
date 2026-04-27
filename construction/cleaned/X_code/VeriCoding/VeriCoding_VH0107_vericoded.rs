use vstd::prelude::*;
verus! {
fn reverse(s: Vec<i8>) -> (rev: Vec<i8>){
    let n = s.len();
    let mut r: Vec<i8> = Vec::new();
    let mut i: usize = 0;
    while i < n
    {
        let idx = n - 1 - i;
        r.push(s[idx]);
        i += 1;
    }
    r
}
}
fn main() {}