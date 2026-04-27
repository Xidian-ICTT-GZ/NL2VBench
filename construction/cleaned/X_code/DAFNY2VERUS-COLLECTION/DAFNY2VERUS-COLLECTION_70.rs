use vstd::prelude::*;
use vstd::seq::*;
verus! {
fn append(a: Vec<int>, b: int) -> (c: Vec<int>){
    let mut c: Vec<int> = Vec::new();
    let mut i: usize = 0;
    while i < a.len()
    {
        c.push(a[i]);
        i += 1;
    }
    c.push(b);
    c
}
fn main() {}
}