use vstd::prelude::*;
use vstd::seq::*;
verus! {
fn remove_front(a: Vec<i32>) -> (c: Vec<i32>){
    let mut c: Vec<i32> = Vec::new();
    let mut i: usize = 1;
    while i < a.len()
    {
        c.push(a[i]);
        i += 1;
    }
    c
}
fn main() {}
}