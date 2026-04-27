use vstd::prelude::*;
verus! {
fn logical_or(x1: Vec<bool>, x2: Vec<bool>) -> (result: Vec<bool>){
    let n = x1.len();
    let mut r: Vec<bool> = Vec::new();
    let mut i: usize = 0;
    while i < n
    {
        let b = x1[i] || x2[i];
        r.push(b);
        i = i + 1;
    }
    r
}
}
fn main() {}