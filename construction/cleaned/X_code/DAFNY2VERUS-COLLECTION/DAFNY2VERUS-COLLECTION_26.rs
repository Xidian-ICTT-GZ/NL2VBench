use vstd::prelude::*;
verus! {
fn array_sum(a: Vec<int>, b: Vec<int>) -> (c: Vec<int>){
    let mut c: Vec<int> = Vec::new();
    let mut i: usize = 0;
    while i < a.len()
    {
        c.push(a[i] + b[i]);
        i += 1;
    }
    c
}
fn main() {}
}