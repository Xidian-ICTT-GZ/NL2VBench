use vstd::prelude::*;
use vstd::seq::*;
verus! {
fn concat(a: Vec<int>, b: Vec<int>) -> (c: Vec<int>){
    let mut c: Vec<int> = Vec::new();
    let mut i: usize = 0;
    while i < a.len()
    {
        c.push(a[i]);
        i += 1;
    }
    i = 0;
    while i < b.len()
    {
        c.push(b[i]);
        i += 1;
    }
    c
}
fn main() {}
}